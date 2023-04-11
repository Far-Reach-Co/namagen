// @deno-types="../../namagen/namagen.d.ts"
import { greet, mamobibu, saurian } from "../../namagen/namagen.js";
import { Router } from "https://deno.land/x/oak@v11.1.0/mod.ts";

const router = new Router();

router.get("/", "You have successfully pinged the Namagen API =)");
router.get("/greet", greet);
router.get("/mamobibu", mamobibu);
router.get("/saurian", saurian);

export default router;
