import { z } from "zod";
import type { RequestHandler } from "./$types";
import { db } from "$lib/server/database";
import { decksToCards } from "$lib/server/schema";
import { and, eq } from "drizzle-orm";

const postRequest = z.object({
    id: z.number(),
})

export const POST: RequestHandler = async ({ request, params }) => {
    const body = postRequest.parse(await request.json());
    const deckId = Number(params.id);

    await db.delete(decksToCards).where(and(eq(decksToCards.deckId, deckId), eq(decksToCards.cardId, body.id)));

    return new Response();
};
