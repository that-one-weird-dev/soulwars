import { db } from "$lib/server/database";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
import { eq } from "drizzle-orm";
import { decks } from "$lib/server/schema";

export const GET: RequestHandler = async ({ params }) => {
    const deck = await db.query.decks.findFirst({
        where: eq(decks.id, +params.id),
        with: {
            decksToCards: {
                columns: { count: true },
                with: {
                    card: true,
                },
            },
        },
    });

    return json(deck);
};
