import init, { run_app } from './pkg/hicard_frontend.js';
async function main() {
   await init('/pkg/hicard_frontend_bg.wasm');
   run_app();
}
main()
