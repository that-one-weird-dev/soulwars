import "./production-network-waiter";
import { Server } from "socket.io";
import { handleConnection } from "./handle-connection";
import {
    ClientToServerEvents,
    ServerToClientEvents,
    SocketData,
} from "./socket-type";
import { env } from "./env";
import { authMiddleware } from "./auth/middleware";

const io = new Server<
    ClientToServerEvents,
    ServerToClientEvents,
    {},
    SocketData
>({
    cors: {
        origin: env.CORS_ORIGIN,
    },
});

io.use(authMiddleware).on("connection", handleConnection);
io.listen(Number(process.env.PORT || 3000), { transports: ["websocket"] });
