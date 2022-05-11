import { rm } from 'node:fs/promises';

const targetDebugFolder = new URL('../target/debug/', import.meta.url);
const targetReleaseFolder = new URL('../target/release/', import.meta.url);

const options = { recursive: true, force: true };

await Promise.all([
	rm(targetDebugFolder, options), //
	rm(targetReleaseFolder, options) //
]);
