import { db } from "$lib/server/database";
import { eq } from "drizzle-orm";
import type { PageServerLoad } from "./$types";
import { decks } from "$lib/server/schema";
import { fail, type Actions } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ parent }) => {
    const { session } = await parent();

    const dbDecks = await db.query.decks.findMany({
        with: {
            decksToCards: {
                columns: { count: true },
                with: {
                    card: true,
                },
            },
        },
        where: eq(decks.userId, session.user?.id ?? ""),
    });

    return {
        decks: dbDecks,
    };
};

export const actions: Actions = {
    create: async ({ locals }) => {
        const session = await locals.getSession();
        if (!session?.user?.id) throw fail(401);

        const deck = await db
            .insert(decks)
            .values({ name: "New deck", userId: session.user.id })
            .returning();

        return { deck };
    },
};
