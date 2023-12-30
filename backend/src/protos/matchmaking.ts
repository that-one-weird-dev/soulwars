/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "";

export interface MatchmakingFind {
  userId: string;
}

export interface MatchmakingFound {
  userId: string;
  matchId: string;
}

export interface MatchmakingCancel {
  userId: string;
}

function createBaseMatchmakingFind(): MatchmakingFind {
  return { userId: "" };
}

export const MatchmakingFind = {
  encode(message: MatchmakingFind, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.userId !== "") {
      writer.uint32(10).string(message.userId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): MatchmakingFind {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseMatchmakingFind();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.userId = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): MatchmakingFind {
    return { userId: isSet(object.userId) ? globalThis.String(object.userId) : "" };
  },

  toJSON(message: MatchmakingFind): unknown {
    const obj: any = {};
    if (message.userId !== "") {
      obj.userId = message.userId;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<MatchmakingFind>, I>>(base?: I): MatchmakingFind {
    return MatchmakingFind.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<MatchmakingFind>, I>>(object: I): MatchmakingFind {
    const message = createBaseMatchmakingFind();
    message.userId = object.userId ?? "";
    return message;
  },
};

function createBaseMatchmakingFound(): MatchmakingFound {
  return { userId: "", matchId: "" };
}

export const MatchmakingFound = {
  encode(message: MatchmakingFound, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.userId !== "") {
      writer.uint32(10).string(message.userId);
    }
    if (message.matchId !== "") {
      writer.uint32(18).string(message.matchId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): MatchmakingFound {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseMatchmakingFound();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.userId = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.matchId = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): MatchmakingFound {
    return {
      userId: isSet(object.userId) ? globalThis.String(object.userId) : "",
      matchId: isSet(object.matchId) ? globalThis.String(object.matchId) : "",
    };
  },

  toJSON(message: MatchmakingFound): unknown {
    const obj: any = {};
    if (message.userId !== "") {
      obj.userId = message.userId;
    }
    if (message.matchId !== "") {
      obj.matchId = message.matchId;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<MatchmakingFound>, I>>(base?: I): MatchmakingFound {
    return MatchmakingFound.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<MatchmakingFound>, I>>(object: I): MatchmakingFound {
    const message = createBaseMatchmakingFound();
    message.userId = object.userId ?? "";
    message.matchId = object.matchId ?? "";
    return message;
  },
};

function createBaseMatchmakingCancel(): MatchmakingCancel {
  return { userId: "" };
}

export const MatchmakingCancel = {
  encode(message: MatchmakingCancel, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.userId !== "") {
      writer.uint32(10).string(message.userId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): MatchmakingCancel {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseMatchmakingCancel();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.userId = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): MatchmakingCancel {
    return { userId: isSet(object.userId) ? globalThis.String(object.userId) : "" };
  },

  toJSON(message: MatchmakingCancel): unknown {
    const obj: any = {};
    if (message.userId !== "") {
      obj.userId = message.userId;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<MatchmakingCancel>, I>>(base?: I): MatchmakingCancel {
    return MatchmakingCancel.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<MatchmakingCancel>, I>>(object: I): MatchmakingCancel {
    const message = createBaseMatchmakingCancel();
    message.userId = object.userId ?? "";
    return message;
  },
};

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
