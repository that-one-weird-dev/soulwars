import { db } from "$lib/server/database";
import { desc, eq } from "drizzle-orm";
import type { PageServerLoad } from "./$types";
import { decks, sessions } from "$lib/server/schema";
import { fail } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ locals }) => {
    const session = await locals.getSession();
    if (!session?.user?.id) {
        throw fail(400);
    }

    const deckList = await db.query.decks.findMany({
        with: {
            decksToCards: {
                with: {
                    card: true,
                },
            },
        },
        where: eq(decks.userId, session.user.id),
    });

    const sessionRecord = await db.query.sessions.findFirst({
        columns: {
            sessionToken: true,
        },
        where: eq(sessions.userId, session.user.id),
        orderBy: [desc(sessions.expires)],
    });

    return {
        decks: deckList,
        sessionToken: sessionRecord?.sessionToken,
    };
};
