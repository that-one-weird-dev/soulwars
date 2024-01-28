import { type GameCard } from "$lib/types/card";
import { writable } from "svelte/store";

export const gameCardOverlay = writable<GameCard | undefined>();
