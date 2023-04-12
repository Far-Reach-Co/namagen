// @deno-types="../../namagen/namagen.d.ts"
import { greet } from "../../namagen/namagen.js";
import { Context } from "https://deno.land/x/oak@v11.1.0/mod.ts";

const index = (ctx: Context, next: Function) => {
  ctx.response.body = "You have successfully pinged the Namagen API =)";
};

const returnGreeting = (ctx: Context, next: Function) => {
  ctx.response.body = greet();
};

export default {
  index,
  returnGreeting,
};
