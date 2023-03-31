import * as wasm from "./namagen_bg.wasm";
import { __wbg_set_wasm } from "./namagen_bg.js";
__wbg_set_wasm(wasm);
export * from "./namagen_bg.js";
