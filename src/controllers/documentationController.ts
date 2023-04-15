// @deno-types="../../pkg/namagen.d.ts"
import { greet } from "../../pkg/namagen.js";
import { Context } from "https://deno.land/x/oak@v11.1.0/mod.ts";

const index = (ctx: Context, next: Function) => {
  ctx.response.body = {
    "result": "You have successfully pinged the Namagen API =)",
  };
};

const returnGreeting = (ctx: Context, next: Function) => {
  ctx.response.body = { "result": greet() };
};

export default {
  index,
  returnGreeting,
};
