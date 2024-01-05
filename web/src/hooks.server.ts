import Discord from "@auth/core/providers/discord";
import { SvelteKitAuth } from "@auth/sveltekit";
import {
    DISCORD_AUTH_ID,
    DISCORD_AUTH_SECRET,
    AUTH_SECRET,
} from "$env/static/private";
import { DrizzleAdapter } from "@auth/drizzle-adapter";
import { db } from "$lib/server/database";
import { redirect, type Handle } from "@sveltejs/kit";
import { sequence } from "@sveltejs/kit/hooks";

export const protectedRoutes = ["/decks", "/match"];

const authorization: Handle = async ({ event, resolve }) => {
    for (const route of protectedRoutes) {
        if (event.url.pathname.startsWith(route)) {
            const session = await event.locals.getSession();
            if (!session) {
                throw redirect(303, "/");
            }
        }
    }

    return resolve(event);
};

const auth = SvelteKitAuth({
    adapter: DrizzleAdapter(db),
    providers: [
        Discord({
            clientId: DISCORD_AUTH_ID,
            clientSecret: DISCORD_AUTH_SECRET,
        }),
    ],
    secret: AUTH_SECRET,
    session: {
        strategy: "database",
    },
    callbacks: {
        session: async (params) => {
            if ("user" in params && params.session.user) {
                params.session.user.id = params.user.id;
            }

            return params.session;
        },
    },
});

export const handle = sequence(auth, authorization);
