import { Router } from "https://deno.land/x/oak@v11.1.0/mod.ts";
import generatorController from "../controllers/generatorController.ts";

const router = new Router();

router.get("/mamobibu", generatorController.genSaurianName);
router.get("/saurian", generatorController.genSaurianName);

export default router;
