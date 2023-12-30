import { publishCancel, publishFind } from "./queues";
import { TypedSocket } from "./socket-type";

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
        });

        publishFind(socket.data.user.id);
    });
}
