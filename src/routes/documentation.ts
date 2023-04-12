import { Router } from "https://deno.land/x/oak@v11.1.0/mod.ts";
import documentationController from "../controllers/documentationController.ts";

const router = new Router();

router.get("/", documentationController.index);
router.get("/greet", documentationController.returnGreeting);

export default router;
