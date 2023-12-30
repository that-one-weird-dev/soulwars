import Discord from "@auth/core/providers/discord";
import { SvelteKitAuth } from "@auth/sveltekit";
import {
    DISCORD_AUTH_ID,
    DISCORD_AUTH_SECRET,
    AUTH_SECRET,
} from "$env/static/private";
import { DrizzleAdapter } from "@auth/drizzle-adapter";
import { db } from "$lib/server/database";

export const handle = SvelteKitAuth({
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
            if ('user' in params && params.session.user) {
                params.session.user.id = params.user.id;
            }

            return params.session;
        },
    },
});
