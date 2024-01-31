import { Socket } from "socket.io";
import { users } from "./db/schema";

export type SocketData = {
    user: typeof users.$inferSelect;
};

export type ClientToServerEvents = {
    "match:find": (callback: (matchData: { matchId: string }) => void) => void;
    "game:start": (deck: number[]) => void;
};

export type ServerToClientEvents = {
    "cant-leave-this-empty": () => void,
};

export type TypedSocket = Socket<ClientToServerEvents, ServerToClientEvents, {}, SocketData>;
