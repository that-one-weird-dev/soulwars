import { db } from "$lib/server/database";
import { eq } from "drizzle-orm";
import type { PageServerLoad } from "./$types";
import { decks } from "$lib/server/schema";
import { fail } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ locals, cookies }) => {
    const session = await locals.getSession();
    if (!session?.user?.id) {
        throw fail(400);
    }

    const deckList = db.query.decks.findMany({
        where: eq(decks.userId, session.user.id),
    });

    return {
        decks: deckList,
        sessionToken: cookies.get("authjs.session-token")!,
    };
};
