// @deno-types="../../pkg/namagen.d.ts"
import { mamobibu, saurian } from "../../pkg/namagen.js";
import { Context } from "https://deno.land/x/oak@v11.1.0/mod.ts";

const genMamobibuName = (ctx: Context, next: Function) => {
  const mamobibuName = mamobibu().split(":")[1];
  ctx.response.body = { "mamobibuName": mamobibuName };
};

const genSaurianName = (ctx: Context, next: Function) => {
  const generatedName = saurian();
  const results = generatedName.split(",");
  const saurianName = results[0].split(":")[1];
  const saurianNameBasicLatin = results[1].split(":")[1];
  ctx.response.body = {
    "saurianName": saurianName,
    "saurianNameBasicLatin": saurianNameBasicLatin,
  };
};

export default {
  genMamobibuName,
  genSaurianName,
};
