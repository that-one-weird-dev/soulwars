import { db } from "$lib/server/database";
import { cards, decksToCards } from "$lib/server/schema";
import { json, type RequestHandler } from "@sveltejs/kit";
import { and, eq, getTableColumns, inArray, isNull, like } from "drizzle-orm";

export const GET: RequestHandler = async ({ url }) => {
    const name = url.searchParams.get("name") ?? "";
    const types = url.searchParams.get("type")?.split(",").map(Number) ?? [
        0, 1, 2,
    ];
    const notInDeck = url.searchParams.get("notInDeck");

    const cardView = notInDeck
        ? await fetchCardsNotInDeck(name, types, Number(notInDeck))
        : await fetchCards(name, types);

    return json(cardView);
};

async function fetchCards(name: string, types: number[]) {
    return await db.query.cards.findMany({
        where: and(
            like(cards.name, `%${name}%`),
            inArray(cards.cardType, types),
        ),
        limit: 16,
    });
}

async function fetchCardsNotInDeck(
    name: string,
    types: number[],
    deckId: number,
) {
    return await db
        .select({
            ...getTableColumns(cards),
        })
        .from(cards)
        .leftJoin(decksToCards, and(eq(cards.id, decksToCards.cardId), eq(decksToCards.deckId, deckId)))
        .where(
            and(
                like(cards.name, `%${name}%`),
                inArray(cards.cardType, types),
                isNull(decksToCards.deckId),
            ),
        );
}
