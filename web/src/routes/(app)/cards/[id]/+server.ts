import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
import { db } from "$lib/server/database";
import { eq } from "drizzle-orm";
import { cards } from "$lib/server/schema";

export const GET: RequestHandler = async ({ params }) => {
    const cardId = Number(params.id);

    return json(
        await db.query.cards.findFirst({ where: eq(cards.id, cardId) }),
    );
};
