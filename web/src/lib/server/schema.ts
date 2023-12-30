import { relations } from "drizzle-orm";
import {
    sqliteTable,
    text,
    integer,
    primaryKey,
} from "drizzle-orm/sqlite-core";
import type { AdapterAccount } from "@auth/core/adapters";

export const cards = sqliteTable("card", {
    id: integer("id").notNull().primaryKey(),

    name: text("name").notNull(),
    description: text("description").notNull(),
    limit: integer("limit").notNull().default(3),
    cardType: integer("card_type").notNull(),

    damage: integer("damage"),
    health: integer("health"),

    enchantmentType: integer("enchantmentType"),
});

export const cardsRelations = relations(cards, ({ many }) => ({
    decksToCards: many(decksToCards),
}));

export const decks = sqliteTable("deck", {
    id: integer("id").notNull().primaryKey({ autoIncrement: true }),

    name: text("name").notNull(),
    userId: text("user_id")
        .notNull()
        .references(() => users.id),
});

export const decksRelations = relations(decks, ({ many, one }) => ({
    decksToCards: many(decksToCards),
    user: one(users, {
        fields: [decks.userId],
        references: [users.id],
    }),
}));

export const decksToCards = sqliteTable(
    "deck_to_card",
    {
        deckId: integer("deck_id")
            .notNull()
            .references(() => decks.id, { onDelete: "cascade" }),
        cardId: integer("card_id")
            .notNull()
            .references(() => cards.id),
        count: integer("count").notNull().default(1),
    },
    (t) => ({
        pk: primaryKey({ columns: [t.deckId, t.cardId] }),
    }),
);

export const decksToCardsRelations = relations(decksToCards, ({ one }) => ({
    deck: one(decks, {
        fields: [decksToCards.deckId],
        references: [decks.id],
    }),
    card: one(cards, {
        fields: [decksToCards.cardId],
        references: [cards.id],
    }),
}));

export const users = sqliteTable("user", {
    id: text("id").notNull().primaryKey(),
    name: text("name"),
    email: text("email").notNull(),
    emailVerified: integer("emailVerified", { mode: "timestamp_ms" }),
    image: text("image"),
});

export const usersRelations = relations(users, ({ many }) => ({
    decks: many(decks),
}));

export const accounts = sqliteTable(
    "account",
    {
        userId: text("userId")
            .notNull()
            .references(() => users.id, { onDelete: "cascade" }),
        type: text("type").$type<AdapterAccount["type"]>().notNull(),
        provider: text("provider").notNull(),
        providerAccountId: text("providerAccountId").notNull(),
        refresh_token: text("refresh_token"),
        access_token: text("access_token"),
        expires_at: integer("expires_at"),
        token_type: text("token_type"),
        scope: text("scope"),
        id_token: text("id_token"),
        session_state: text("session_state"),
    },
    (account) => ({
        compoundKey: primaryKey({
            columns: [account.provider, account.providerAccountId],
        }),
    }),
);

export const sessions = sqliteTable("session", {
    sessionToken: text("sessionToken").notNull().primaryKey(),
    userId: text("userId")
        .notNull()
        .references(() => users.id, { onDelete: "cascade" }),
    expires: integer("expires", { mode: "timestamp_ms" }).notNull(),
});

export const verificationTokens = sqliteTable(
    "verificationToken",
    {
        identifier: text("identifier").notNull(),
        token: text("token").notNull(),
        expires: integer("expires", { mode: "timestamp_ms" }).notNull(),
    },
    (vt) => ({
        compoundKey: primaryKey({ columns: [vt.identifier, vt.token] }),
    }),
);
