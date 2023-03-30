import init, { main, greet } from "./pkg/namagen.js";

init().then(() => {
  const exampleText = document.getElementById("exampleText");
  exampleText.textContent = greet();
});
