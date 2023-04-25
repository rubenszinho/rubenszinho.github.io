import init, { navigate_to_page } from "./pkg/navigation.js";

async function run() {
  await init();
  navigate_to_page('home')
  window.navigate_to_page = navigate_to_page;
}

run();
