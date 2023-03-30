import init, { main, greet } from "./pkg/namagen.js";

init().then(() => {
  const greeting = document.getElementById("greeting");
  const message = document.getElementById("message");
  const userInput = document.getElementById("userInput");
  const enterButton = document.getElementById("enterButton");

  //   greeting.textContent = greet();

  userInput.addEventListener("keydown", (event) => {
    if (event.code === "Enter") {
      message.textContent = main(userInput.value);
    }
  });

  enterButton.addEventListener("click", () => {
    message.textContent = main(userInput.value);
  });
});
