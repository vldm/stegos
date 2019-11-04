import init, { run_app } from './pkg/blockchain_explorer.js';
async function main() {
   await init('/pkg/blockchain_explorer_bg.wasm');
   run_app();
}
main()