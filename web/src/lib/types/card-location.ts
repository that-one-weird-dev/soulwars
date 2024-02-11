import type { FieldSlot } from "./field-slot";

export type CardLocation =
    | { type: "field"; slot: FieldSlot }
    | { type: "hand"; index: number }
    | { type: "graveyard"; index: number }
    | { type: "deck"; index: number };
