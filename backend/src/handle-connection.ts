import { io } from "socket.io-client";
import { publishCancel, publishFind } from "./queues";
import { TypedSocket } from "./socket-type";
import { env } from "./env";

export const userCallbackMap = new Map<string, (matchId: string) => void>();

export async function handleConnection(socket: TypedSocket) {
    console.log(" [*] WS connected");

    socket.on("disconnect", () => {
        const callback = userCallbackMap.get(socket.data.user.id);
        if (callback) {
            userCallbackMap.delete(socket.data.user.id);

            publishCancel(socket.data.user.id);
        }
    });

    socket.on("match:find", (callback) => {
        userCallbackMap.set(socket.data.user.id, (matchId) => {
            userCallbackMap.delete(socket.data.user.id);

            callback({ matchId });

            socket.on("game:start", (deck) => {
                handleGameStart(socket, socket.data.user.id, deck);
            });
        });

        publishFind(socket.data.user.id);
    });
}

async function handleGameStart(socket: TypedSocket, userId: string, deck: number[]) {
    const gameServerSocket = io(env.GAME_SERVER_URL, {
        auth: {
            userId: userId,
            gameId: "5c1ab66f-d453-4285-82fc-963fb6096947",
            deck,
        },
        transports: ["websocket"],
    });

    gameServerSocket.on("connect", () => {
        console.log("connected to game server");
    });

    gameServerSocket.onAny(async (event, ...args) => {
        console.log(`game server sent ${event}`);

        const lastArg = args[args.length - 1];
        if (lastArg instanceof Function) {
            const response = await socket.emitWithAck(event, ...(args.toSpliced(args.length - 1, 1)));
            console.log(`client responded to ${event} with ${response}`);

            lastArg(response);
        } else {
            socket.emit(event, ...args);
        }
    });

    socket.onAny(async (event, ...args) => {
        const lastArg = args[args.length - 1];
        if (lastArg instanceof Function) {
            console.log(`emitting ${event} with ack`)
            const response = await gameServerSocket.emitWithAck(event, ...(args.toSpliced(args.length - 1, 1)));

            lastArg(response);
        } else {
            console.log(`emitting ${event}`);
            gameServerSocket.emit(event, ...args);
        }
    });
}
