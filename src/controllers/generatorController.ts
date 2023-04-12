// @deno-types="../../namagen/namagen.d.ts"
import { mamobibu, saurian } from "../../namagen/namagen.js";
import { Context } from "https://deno.land/x/oak@v11.1.0/mod.ts";

const genMamobibuName = (ctx: Context, next: Function) => {
  ctx.response.body = mamobibu();
};

const genSaurianName = (ctx: Context, next: Function) => {
  ctx.response.body = saurian();
};

export default {
  genMamobibuName,
  genSaurianName,
};
