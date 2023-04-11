import { Application, Context } from "https://deno.land/x/oak/mod.ts";
import { config } from "https://deno.land/x/dotenv/mod.ts";

const port = parseInt(config()["PORT"]);

const app = new Application();

const logging = async (ctx: Context, next: Function) => {
  console.log(`HTTP ${ctx.request.method} on ${ctx.request.url}`);
  console.log("Hello Deno!!");
  console.log("returning a response ...");
  await next();
};

app.use(logging);

app.addEventListener("listen", () => {
  console.log(`Listening on localhost:${port}`);
});

await app.listen({ port });
