import init, { run_app } from './pkg/yew_todo.js';
async function main() {
  await init('/pkg/yew_todo_bg.wasm');
  run_app();
}
main()