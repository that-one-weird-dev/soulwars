import { db } from "$lib/server/database";
import { decks } from "$lib/server/schema";
import { fail } from "@sveltejs/kit";
import { and, eq } from "drizzle-orm";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
    const id = Number(params.id);

    const deck = await db.query.decks.findFirst({
        where: eq(decks.id, id),
        with: {
            decksToCards: {
                columns: { count: true },
                with: {
                    card: true,
                },
            },
        },
    });

    if (!deck) {
        throw fail(400);
    }

    return {
        deck,
    };
};

export const actions: Actions = {
    delete: async ({ locals, params }) => {
        const session = await locals.getSession();
        if (!session?.user?.id) throw fail(401);

        const id = Number(params.id);

        await db
            .delete(decks)
            .where(and(eq(decks.id, id), eq(decks.userId, session.user.id)));
    },
    changeName: async ({ locals, params, request }) => {
        const session = await locals.getSession();
        if (!session?.user?.id) throw fail(401);

        const data = await request.formData();

        const name = data.get("name")?.toString() ?? "New deck";
        const id = Number(params.id);

        await db.update(decks).set({ name }).where(eq(decks.id, id));
    },
};
