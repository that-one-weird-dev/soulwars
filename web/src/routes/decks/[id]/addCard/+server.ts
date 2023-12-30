import { db } from "$lib/server/database";
import { decksToCards } from "$lib/server/schema";
import { z } from "zod";
import type { RequestHandler } from "./$types";

const postRequest = z.object({
    id: z.number(),
});

export const POST: RequestHandler = async ({ params, request }) => {
    const deckId = Number(params.id);

    const body = postRequest.parse(await request.json());

    await db
        .insert(decksToCards)
        .values({
            cardId: body.id,
            deckId,
        })
        .onConflictDoNothing({ target: [decksToCards.cardId, decksToCards.deckId] });

    return new Response();
};
