import { connect } from "amqplib";
import { env } from "../env";
import {
    MatchmakingCancel,
    MatchmakingFind,
    MatchmakingFound,
} from "../protos/matchmaking";
import { userCallbackMap } from "../handle-connection";

console.log(" [*] Connecting to amqp")

const connection = await connect(env.RABBIT_URL);

console.log(" [*] Conntected to amqp")

export const channel = await connection.createChannel();

export const findQueue = "matchmaking.find";
export const cancelQueue = "matchmaking.cancel";

await channel.assertQueue(findQueue, { durable: false });
const { queue: foundQueue } = await channel.assertQueue("", {
    durable: false,
    exclusive: true,
});

channel.consume(
    foundQueue,
    (message) => {
        if (!message) return;

        console.log("match found");

        const request = MatchmakingFound.decode(
            Uint8Array.from(message.content),
        );
        const callback = userCallbackMap.get(request.userId);

        callback?.(request.matchId);
    },
    { noAck: true },
);

export function publishFind(userId: string) {
    const request = MatchmakingFind.encode({
        userId,
    }).finish();

    channel.publish("", findQueue, Buffer.from(request), {
        replyTo: foundQueue,
    });
}

export function publishCancel(userId: string) {
    const request = MatchmakingCancel.encode({
        userId,
    }).finish();

    channel.publish("", cancelQueue, Buffer.from(request));
}
