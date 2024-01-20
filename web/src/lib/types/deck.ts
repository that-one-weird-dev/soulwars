import type { GameCard } from "./card";

export type GameDeck = {
    id: number;
    name: string;
    decksToCards: {
        count: number,
        card: GameCard,
    }[],
}
