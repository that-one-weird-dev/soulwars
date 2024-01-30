import { z } from "zod";
import type { RequestHandler } from "./$types";
import { db } from "$lib/server/database";
import { cards, decksToCards } from "$lib/server/schema";
import { and, eq } from "drizzle-orm";
import { fail } from "assert";

const postRequest = z.object({
    id: z.number(),
    count: z.number(),
});

export const POST: RequestHandler = async ({ request, params }) => {
    const body = postRequest.parse(await request.json());
    const deckId = Number(params.id);

    const card = await db.query.cards.findFirst({
        where: eq(cards.id, body.id),
    });

    if (!card) {
        throw fail("Invalid card");
    }

    if (body.count > card.limit) {
        throw fail("Invalid card count");
    }

    await db
        .update(decksToCards)
        .set({ count: body.count })
        .where(
            and(
                eq(decksToCards.deckId, deckId),
                eq(decksToCards.cardId, body.id),
            ),
        );

    return new Response();
};
