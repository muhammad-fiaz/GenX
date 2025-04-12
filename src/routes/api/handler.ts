import type { RequestHandler } from '@sveltejs/kit';
import * as fs from 'fs';
import * as path from 'path';

// Path to the Ollama configuration file
const OLLAMA_CONFIG_PATH = path.resolve('models', 'ollama.json');

// Helper function to read the Ollama configuration file
const getConfig = (): Record<string, never> => {
	try {
		const configData = fs.readFileSync(OLLAMA_CONFIG_PATH, 'utf8');
		return JSON.parse(configData);
	} catch  {
		throw new Error('Failed to read Ollama configuration file.');
	}
};

// Helper function to make a request to the Ollama API
const makeOllamaRequest = async (apiUrl: string, payload: Record<string, unknown>) => {
	const response = await fetch(apiUrl, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(payload)
	});

	if (!response.ok) {
		throw new Error(`Ollama API request failed with status ${response.status}`);
	}

	return await response.json();
};

export const POST: RequestHandler = async ({ request }) => {
	try {
		// Parse incoming request data
		const requestData = await request.json();

		// Read configuration from the JSON file
		const config = getConfig();

		// Validate the required fields in the request
		if (!requestData.model || !requestData.prompt) {
			return new Response(
				JSON.stringify({ error: 'Missing required fields: "model" and "prompt"' }),
				{ status: 400, headers: { 'Content-Type': 'application/json' } }
			);
		}

		// Prepare the payload for the Ollama API request
		const payload: Record<string, unknown> = {
			model: requestData.model,
			prompt: requestData.prompt,
			stream: requestData.stream ?? false, // Default to non-streaming
			suffix: requestData.suffix ?? undefined,
			format: requestData.format ?? undefined,
			options: requestData.options ?? {}
		};

		// Make the request to the Ollama API
		const apiResponse = await makeOllamaRequest(`${config.api_url}/api/generate`, payload);

		// Return the response from the Ollama API
		return new Response(JSON.stringify(apiResponse), {
			status: 200,
			headers: { 'Content-Type': 'application/json' }
		});
	} catch (error) {
		console.error('Error in Ollama handler:', error);

		return new Response(
			JSON.stringify({ error: 'An error occurred while processing the request.' }),
			{ status: 500, headers: { 'Content-Type': 'application/json' } }
		);
	}
};