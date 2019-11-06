import init, { run_app } from './ev/blockchain_explorer.js';
async function main() {
   await init('/ev/blockchain_explorer_bg.wasm');
   run_app();
}
main()