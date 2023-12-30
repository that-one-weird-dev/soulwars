import { Server } from "socket.io";
import { getSession } from "./session";

export const authMiddleware: Parameters<Server["use"]>[0] = async (socket, next) => {
    const session = await getSession(socket.handshake.auth);
    if (!session?.user) {
        return;
    }

    socket.data.user = session.user;

    next();
}
