import { fail, json, type RequestHandler } from "@sveltejs/kit";

export const GET: RequestHandler = async ({ cookies, locals }) => {
    const session = await locals.getSession();
    if (!session?.user) throw fail(401);

    return json({
        sessionId: cookies.get("authjs.session-token"),
    });
};
