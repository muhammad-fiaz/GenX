import { existsSync, writeFileSync } from 'fs';
import { generateKeyPairSync } from 'crypto';
import chalk from 'chalk';

const pemPath = './extension.pem';

if (existsSync(pemPath)) {
	console.log(chalk.yellow(`⚠ ${pemPath} already exists, skipping generation.`));
	process.exit(0);
}

try {
	const { privateKey } = generateKeyPairSync('rsa', {
		modulusLength: 2048,
		privateKeyEncoding: {
			type: 'pkcs8',
			format: 'pem'
		},
		publicKeyEncoding: {
			type: 'spki',
			format: 'pem'
		}
	});

	writeFileSync(pemPath, privateKey);

	console.log(chalk.greenBright(`✔ ${pemPath} generated successfully!`));
	console.log(
		chalk.blue(`
  Use this .pem file to sign your Chrome extension when packing it:
  
  crx pack ./your-build-folder -o ./your-extension.crx -p ./extension.pem
  `)
	);
} catch (err) {
	console.error(chalk.redBright('✖ Failed to generate extension.pem:'), err);
	process.exit(1);
}
