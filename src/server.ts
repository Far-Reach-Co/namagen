import { Application, Context } from "https://deno.land/x/oak@v11.1.0/mod.ts";

import routes from "./routes/index.ts";

const port = 8000;

const app = new Application();

const logging = async (ctx: Context, next: Function) => {
  console.log(`HTTP ${ctx.request.method} on ${ctx.request.url}`);
  console.log("Hello Deno!!");
  console.log("returning a response ...");
  await next();
};

app.use(logging);

app.use(routes.documentation.allowedMethods());
app.use(routes.documentation.routes());
app.use(routes.generator.allowedMethods());
app.use(routes.generator.routes());

app.addEventListener("listen", () => {
  console.log(`Listening on localhost:${port}`);
});

await app.listen({ port });
