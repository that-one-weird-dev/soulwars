import { z } from "zod";
import { db } from "../db";
import { eq } from "drizzle-orm";
import { sessions } from "../db/schema";

export const authRequest = z.object({
    sessionToken: z.string(),
});

export async function getSession(authObj: any) {
    const result = authRequest.safeParse(authObj);
    if (!result.success) {
        return undefined;
    }

    const auth = result.data;

    return await db.query.sessions.findFirst({
        where: eq(sessions.sessionToken, auth.sessionToken),
        with: { user: true },
    });
}
