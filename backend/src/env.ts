import { z } from "zod";

export const env = z
    .object({
        PORT: z.string().min(1).default("3000"),

        DATABASE_URL: z.string().min(1),
        DATABASE_AUTH_TOKEN: z.string().min(1),

        CORS_ORIGIN: z.string().min(1),

        RABBIT_URL: z.string().min(1),
        GAME_SERVER_URL: z.string().min(1),
    })
    .parse(process.env);
