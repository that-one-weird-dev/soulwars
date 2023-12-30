export enum CardType {
    Yokai,
    Artifact,
    Object,
    Enchantment,
    Terrain,
}

export function cardTypeList() {
    return [
        CardType.Yokai,
        CardType.Artifact,
        CardType.Object,
        CardType.Enchantment,
        CardType.Terrain,
    ];
}

export enum CardEnchantmentType {
    Normal,
    Blessing,
    Curse,
}

export type GameCardYokai = {
    cardType: CardType.Yokai;
    damage: number;
    health: number;
};

export type GameCardEnchantment = {
    cardType: CardType.Enchantment;
    enchantmentType: CardEnchantmentType;
};

export type GameCardOther = {
    cardType: CardType.Artifact | CardType.Object | CardType.Terrain;
};

export type GameCard = {
    id: number;
    name: string;
    description: string;
} & (GameCardYokai | GameCardEnchantment | GameCardOther);
