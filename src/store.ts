import { Writable, writable } from "svelte/store";
import type { Response } from "./types";

export const queryResult:Writable<Response[]> = writable([]);
export const err:Writable<string> = writable("");