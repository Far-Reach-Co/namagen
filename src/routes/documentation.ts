// @deno-types="../../namagen/namagen.d.ts"
import { greet } from "../../namagen/namagen.js";
import { Router } from "https://deno.land/x/oak@v11.1.0/mod.ts";

const router = new Router();

router.get("/", "You have successfully pinged the Namagen API =)");
router.get("/greet", greet);

export default router;
