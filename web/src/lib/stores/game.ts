import type { Socket } from "socket.io-client";
import { writable } from "svelte/store";

const socket = writable<Socket | undefined>();
