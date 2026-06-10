pub enum TypeName {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

pub struct Type<'a> {
    type_name: TypeName,

    // Defensive
    Resistances: &'a [TypeName],
    Immunities: &'a [TypeName],
    Weaknesses: &'a [TypeName],

    // Offensive
    V_Effective: &'a [TypeName],
    N_Effective: &'a [TypeName],
    NO_EFFECT: &'a [TypeName],
}

pub struct DualType<'a> {
    type_name: (TypeName, TypeName),
    Resistance: &'a [TypeName],
    Immunities: &'a [TypeName],
    V_Effective: &'a [TypeName],
    N_Effective: &'a [TypeName],
    E_EffectivVe: &'a [TypeName],
    EN_EffectivVe: &'a [TypeName],
}

impl Type<'_> {
    pub fn eval_type() -> DualType<'static> {

        todo!();
    }
}

const normal_name: TypeName = TypeName::Normal;
const fire_name: TypeName = TypeName::Fire;
const water_name: TypeName = TypeName::Water;
const electric_name: TypeName = TypeName::Electric;
const grass_name: TypeName = TypeName::Grass;
const ice_name: TypeName = TypeName::Ice;
const fighting_name: TypeName = TypeName::Fighting;
const poison_name: TypeName = TypeName::Poison;
const ground_name: TypeName = TypeName::Ground;
const flying_name: TypeName = TypeName::Flying;
const psychic_name: TypeName = TypeName::Psychic;
const bug_name: TypeName = TypeName::Bug;
const rock_name: TypeName = TypeName::Rock;
const ghost_name: TypeName = TypeName::Ghost;
const dragon_name: TypeName = TypeName::Dragon;
const dark_name: TypeName = TypeName::Dark;
const steel_name: TypeName = TypeName::Steel;
const fairy_name: TypeName = TypeName::Fairy;

const NORMAL: Type = Type {
    type_name: normal_name,
    // Defensive
    Resistances: &[],
    Immunities: &[ghost_name],
    Weaknesses: &[fighting_name],
    // Offensive
    V_Effective: &[],
    N_Effective: &[rock_name, steel_name],
    NO_EFFECT: &[ghost_name],
};

const FIRE: Type = Type {
    type_name: fire_name,
    // Defensive
    Resistances: &[fire_name, grass_name, ice_name, bug_name, steel_name, fairy_name],
    Immunities: &[],
    Weaknesses: &[water_name, ground_name, rock_name],
    // Offensive
    V_Effective: &[grass_name, ice_name, bug_name, steel_name],
    N_Effective: &[fire_name, water_name, rock_name, dragon_name],
    NO_EFFECT: &[],
};

const WATER: Type = Type {
    type_name: water_name,
    // Defensive
    Resistances: &[fire_name, water_name, ice_name, steel_name],
    Immunities: &[],
    Weaknesses: &[electric_name, grass_name],
    // Offensive
    V_Effective: &[fire_name, ground_name, rock_name],
    N_Effective: &[water_name, grass_name, dragon_name],
    NO_EFFECT: &[],
};

const ELECTRIC: Type = Type {
    type_name: electric_name,
    // Defensive
    Resistances: &[electric_name, flying_name, steel_name],
    Immunities: &[],
    Weaknesses: &[ground_name],
    // Offensive
    V_Effective: &[water_name, flying_name],
    N_Effective: &[electric_name, grass_name, dragon_name],
    NO_EFFECT: &[ground_name],
};

pub const GRASS: Type = Type {
    type_name: grass_name,
    // Defensive
    Resistances: &[water_name, electric_name, grass_name, ground_name],
    Immunities: &[],
    Weaknesses: &[fire_name, ice_name, poison_name, flying_name, bug_name],
    // Offensive
    V_Effective: &[water_name, ground_name, rock_name],
    N_Effective: &[fire_name, grass_name, poison_name, flying_name, bug_name, dragon_name, steel_name],
    NO_EFFECT: &[],
};

const ICE: Type = Type {
    type_name: ice_name,
    // Defensive
    Resistances: &[ice_name],
    Immunities: &[],
    Weaknesses: &[fire_name, fighting_name, rock_name, steel_name],
    // Offensive
    V_Effective: &[flying_name, ground_name, grass_name, dragon_name],
    N_Effective: &[fire_name, water_name, ice_name, steel_name],
    NO_EFFECT: &[],
};

const FIGHTING: Type = Type {
    type_name: fighting_name,
    // Defensive
    Resistances: &[bug_name, rock_name, dark_name],
    Immunities: &[ghost_name],
    Weaknesses: &[flying_name, psychic_name, fairy_name],
    // Offensive
    V_Effective: &[normal_name, ice_name, rock_name, dark_name, steel_name],
    N_Effective: &[poison_name, flying_name, psychic_name, bug_name, fairy_name],
    NO_EFFECT: &[ghost_name],
};

const POISON: Type = Type {
    type_name: poison_name,
    // Defensive
    Resistances: &[fighting_name, poison_name, bug_name, grass_name, fairy_name],
    Immunities: &[steel_name],
    Weaknesses: &[ground_name, psychic_name],
    // Offensive
    V_Effective: &[grass_name, fairy_name],
    N_Effective: &[poison_name, ground_name, rock_name, ghost_name, steel_name],
    NO_EFFECT: &[steel_name],
};

const GROUND: Type = Type {
    type_name: ground_name,
    // Defensive
    Resistances: &[poison_name, rock_name],
    Immunities: &[electric_name],
    Weaknesses: &[water_name, grass_name, ice_name],
    // Offensive
    V_Effective: &[fire_name, electric_name, poison_name, rock_name, steel_name],
    N_Effective: &[grass_name, bug_name],
    NO_EFFECT: &[flying_name],
};

const FLYING: Type = Type {
    type_name: flying_name,
    // Defensive
    Resistances: &[fighting_name, bug_name, grass_name],
    Immunities: &[ground_name],
    Weaknesses: &[electric_name, ice_name, rock_name],
    // Offensive
    V_Effective: &[grass_name, fighting_name, bug_name],
    N_Effective: &[electric_name, rock_name, steel_name],
    NO_EFFECT: &[],
};

const PSYCHIC: Type = Type {
    type_name: psychic_name,
    // Defensive
    Resistances: &[fighting_name, psychic_name],
    Immunities: &[],
    Weaknesses: &[bug_name, ghost_name, dark_name],
    // Offensive
    V_Effective: &[fighting_name, poison_name],
    N_Effective: &[psychic_name, steel_name],
    NO_EFFECT: &[dark_name],
};

const BUG: Type = Type {
    type_name: bug_name,
    // Defensive
    Resistances: &[fighting_name, ground_name, grass_name],
    Immunities: &[],
    Weaknesses: &[fire_name, flying_name, rock_name],
    // Offensive
    V_Effective: &[grass_name, psychic_name, dark_name],
    N_Effective: &[fire_name, fighting_name, poison_name, flying_name, ghost_name, steel_name, fairy_name],
    NO_EFFECT: &[],
};

const ROCK: Type = Type {
    type_name: rock_name,
    // Defensive
    Resistances: &[normal_name, fire_name, poison_name, flying_name],
    Immunities: &[],
    Weaknesses: &[water_name, grass_name, fighting_name, ground_name, steel_name],
    // Offensive
    V_Effective: &[fire_name, ice_name, flying_name, bug_name],
    N_Effective: &[fighting_name, ground_name, steel_name],
    NO_EFFECT: &[],
};

const GHOST: Type = Type {
    type_name: ghost_name,
    // Defensive
    Resistances: &[poison_name, bug_name],
    Immunities: &[normal_name, fighting_name],
    Weaknesses: &[ghost_name, dark_name],
    // Offensive
    V_Effective: &[psychic_name, ghost_name],
    N_Effective: &[dark_name, steel_name],
    NO_EFFECT: &[normal_name],
};

const DRAGON: Type = Type {
    type_name: dragon_name,
    // Defensive
    Resistances: &[fire_name, water_name, electric_name, grass_name],
    Immunities: &[],
    Weaknesses: &[ice_name, dragon_name, fairy_name],
    // Offensive
    V_Effective: &[dragon_name],
    N_Effective: &[steel_name],
    NO_EFFECT: &[fairy_name],
};

const DARK: Type = Type {
    type_name: dark_name,
    // Defensive
    Resistances: &[ghost_name, dark_name],
    Immunities: &[psychic_name],
    Weaknesses: &[fighting_name, bug_name, fairy_name],
    // Offensive
    V_Effective: &[psychic_name, ghost_name],
    N_Effective: &[fighting_name, dark_name, fairy_name],
    NO_EFFECT: &[],
};

const STEEL: Type = Type {
    type_name: steel_name,
    // Defensive
    Resistances: &[
        normal_name, grass_name, ice_name, flying_name,
        psychic_name, bug_name, rock_name, dragon_name,
        steel_name, fairy_name
    ],
    Immunities: &[poison_name],
    Weaknesses: &[fire_name, fighting_name, ground_name],
    // Offensive
    V_Effective: &[ice_name, rock_name, fairy_name],
    N_Effective: &[fire_name, water_name, electric_name, steel_name],
    NO_EFFECT: &[],
};

const FAIRY: Type = Type {
    type_name: fairy_name,
    // Defensive
    Resistances: &[fighting_name, bug_name, dark_name],
    Immunities: &[dragon_name],
    Weaknesses: &[poison_name, steel_name],
    // Offensive
    V_Effective: &[fighting_name, dragon_name, dark_name],
    N_Effective: &[fire_name, poison_name, steel_name],
    NO_EFFECT: &[],
};
