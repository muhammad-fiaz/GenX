use dotenv::dotenv;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::async_runtime::Mutex;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const REDIRECT_PORT: u16 = 5174;
const REDIRECT_URI: &str = "http://localhost:5174/callback";

#[derive(Serialize, Deserialize)]
pub struct GithubUser {
    pub id: u64,
    pub login: String,
    pub avatar_url: String,
    pub node_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[tauri::command]
pub async fn github_login(_window: tauri::Window) -> Result<GithubUser, String> {
    dotenv().ok();

    let client_id = std::env::var("GITHUB_CLIENT_ID").map_err(|_| "GITHUB_CLIENT_ID not set".to_string())?;
    let client_secret = std::env::var("GITHUB_CLIENT_SECRET").map_err(|_| "GITHUB_CLIENT_SECRET not set".to_string())?;

    let redirect_uri_enc = percent_encode(REDIRECT_URI.as_bytes(), NON_ALPHANUMERIC).to_string();
    let scope_enc = percent_encode(b"read:user,user:email", NON_ALPHANUMERIC).to_string();

    let auth_url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}",
        client_id, redirect_uri_enc, scope_enc
    );

    if open::that(&auth_url).is_err() {
        return Err("Failed to open browser for OAuth".into());
    }

    // Start the async TCP listener
    let listener = TcpListener::bind(format!("127.0.0.1:{}", REDIRECT_PORT))
        .await
        .map_err(|e| format!("Failed to bind local port: {e}"))?;

    let code: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let code_ref = code.clone();

    // Accept one connection for the callback (non-blocking)
    let handle = tokio::spawn(async move {
        if let Ok((mut socket, _)) = listener.accept().await {
            let mut buffer = [0; 2048];
            if let Ok(size) = socket.read(&mut buffer).await {
                let request = String::from_utf8_lossy(&buffer[..size]);
                if request.starts_with("GET /callback?") {
                    if let Some(params) = request.split_whitespace().nth(1).and_then(|path| path.split('?').nth(1)) {
                        let code_val = params.split('&')
                            .find_map(|kv| {
                                let mut split = kv.splitn(2, '=');
                                match (split.next(), split.next()) {
                                    (Some("code"), Some(value)) => Some(value.to_string()),
                                    _ => None,
                                }
                            });

                        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
                            <html><body><h1>Login successful! You may close this window.</h1></body></html>";
                        let _ = socket.write_all(response.as_bytes()).await;

                        if let Some(code_val) = code_val {
                            let mut code_lock = code_ref.lock().await;
                            *code_lock = Some(code_val);
                        }
                    }
                }
            }
        }
    });

    // Wait for the code to be set by the TCP handler above
    let code_val = loop {
        {
            let code_lock = code.lock().await;
            if let Some(ref code) = *code_lock {
                break code.clone();
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    };

    handle.await.ok(); // clean up

    // Exchange code for access token
    let client = Client::new();
    let token_res = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code_val.as_str()),
            ("redirect_uri", REDIRECT_URI),
        ])
        .send()
        .await
        .map_err(|e| format!("Failed to exchange code for token: {e}"))?;

    #[derive(Deserialize)]
    struct GithubToken {
        access_token: String,
    }
    let token: GithubToken = token_res.json().await.map_err(|e| format!("Failed to parse token: {e}"))?;

    // Get user info from GitHub API
    let user_res = client
        .get("https://api.github.com/user")
        .bearer_auth(&token.access_token)
        .header("User-Agent", "GenX")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch user info: {e}"))?;

    let mut user: GithubUser = user_res.json().await.map_err(|e| format!("Failed to parse user: {e}"))?;

    // If email is missing, attempt to fetch from /user/emails endpoint
    if user.email.is_none() {
        let emails_res = client
            .get("https://api.github.com/user/emails")
            .bearer_auth(&token.access_token)
            .header("User-Agent", "GenX")
            .send()
            .await;
        if let Ok(emails_res) = emails_res {
            if let Ok(emails) = emails_res.json::<Vec<serde_json::Value>>().await {
                if let Some(default_email) = emails.iter().find_map(|e| e.get("email").and_then(|v| v.as_str())) {
                    user.email = Some(default_email.to_string());
                }
            }
        }
    }

    Ok(user)
}