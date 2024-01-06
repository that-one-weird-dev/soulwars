import { Socket } from "socket.io";
import { users } from "./db/schema";

export type SocketData = {
    user: typeof users.$inferSelect;
};

export type ClientToServerEvents = {
    "match:find": (callback: (matchData: { matchId: string }) => void) => void;
};

export type ServerToClientEvents = {
    "match:found": () => void;
    "match:ready": () => void;
};

export type TypedSocket = Socket<ClientToServerEvents, ServerToClientEvents, {}, SocketData>;
