import { db } from "$lib/server/database";
import { and, inArray, like } from "drizzle-orm";
import type { PageServerLoad } from "./$types";
import { cards } from "$lib/server/schema";

export const load: PageServerLoad = async ({ url }) => {
    const name = url.searchParams.get("name") ?? "";
    const types = url.searchParams.get("type")?.split(",").map(Number) ?? [
        0, 1, 2,
    ];

    const cardView = await db.query.cards.findMany({
        where: and(
            like(cards.name, `%${name}%`),
            inArray(cards.cardType, types),
        ),
        limit: 16,
    });
    
    return {
        cards: cardView,
    }
};
