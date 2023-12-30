import type { GameCard } from "./card";

export type GameDeck = {
    id: number;
    name: string;
    cards: GameCard[];
}
