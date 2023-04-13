// @deno-types="../../namagen/namagen.d.ts"
import { mamobibu, saurian } from "../../namagen/namagen.js";
import { Context } from "https://deno.land/x/oak@v11.1.0/mod.ts";

const genMamobibuName = (ctx: Context, next: Function) => {
  ctx.response.body = { "result": mamobibu() };
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
