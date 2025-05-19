import { execSync } from 'child_process';
import * as fs from 'fs';
import chalk from 'chalk';

function getBuildFolder(): string {
	const folders = fs
		.readdirSync('.', { withFileTypes: true })
		.filter((dirent) => dirent.isDirectory() && dirent.name.startsWith('genx-build-'))
		.map((dirent) => dirent.name);

	if (folders.length === 0) {
		throw new Error('No genx-build-* folder found');
	}
	return folders[0];
}

try {
	const buildFolder = getBuildFolder();
	const exportFolder = './export';
	const privateKey = './extension.pem';

	if (!fs.existsSync(privateKey)) {
		console.error(chalk.redBright('✖ Error: extension.pem file not found in the root directory.'));
		process.exit(1);
	}

	if (!fs.existsSync(exportFolder)) {
		fs.mkdirSync(exportFolder);
	}

	const outputFile = `${exportFolder}/${buildFolder}.crx`;

	// Run crx command globally, redirect stderr to null to hide warnings
	execSync(`crx pack ${buildFolder} -o ${outputFile} -p ${privateKey} 2>nul`, {
		stdio: 'inherit',
		shell: true
	} as import('child_process').ExecSyncOptions);
	console.log(
		chalk.greenBright(`✔ Extension packed successfully from ${buildFolder} to ${outputFile}`)
	);
} catch (error) {
	console.error(chalk.redBright('✖ Error during extension packing:'), error);
	process.exit(1);
}
