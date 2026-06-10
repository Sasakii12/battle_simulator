enum TypeName {
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

struct Type<'a> {
    type_name: TypeName,
    Resistance: &'a [TypeName],
    Immunities: &'a [TypeName],
    V_Effective: &'a [TypeName],
    N_Effective: &'a [TypeName],
}

struct DualType<'a> {
    type_name: (TypeName, TypeName),
    Resistance: &'a [TypeName],
    Immunities: &'a [TypeName],
    V_Effective: &'a [TypeName],
    N_Effective: &'a [TypeName],
    E_EffectivVe: &'a [TypeName],
    EN_EffectivVe: &'a [TypeName],
}

impl Type<'_> {
    fn eval_type() -> DualType<'static> {

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
    Resistance: &[],
    Immunities: &[ghost_name],
    V_Effective: &[fighting_name],
    N_Effective: &[],
};

const FIRE: Type = Type {
    type_name: fire_name,
    Resistance: &[fire_name, grass_name, ice_name, bug_name, steel_name, fairy_name],
    Immunities: &[],
    V_Effective: &[water_name, ground_name, rock_name],
    N_Effective: &[fire_name, grass_name, ice_name, bug_name, steel_name, fairy_name],
};

const WATER: Type = Type {
    type_name: water_name,
    Resistance: &[fire_name, water_name, ice_name, steel_name],
    Immunities: &[],
    V_Effective: &[electric_name, grass_name],
    N_Effective: &[fire_name, water_name, ice_name, steel_name],
};

const ELECTRIC: Type = Type {
    type_name: electric_name,
    Resistance: &[electric_name, flying_name, steel_name],
    Immunities: &[],
    V_Effective: &[ground_name],
    N_Effective: &[electric_name, flying_name, steel_name],
};

const GRASS: Type = Type {
    type_name: grass_name,
    Resistance: &[water_name, electric_name, grass_name, ground_name],
    Immunities: &[],
    V_Effective: &[fire_name, ice_name, poison_name, flying_name, bug_name],
    N_Effective: &[water_name, electric_name, grass_name, ground_name],
};

const ICE: Type = Type {
    type_name: ice_name,
    Resistance: &[ice_name],
    Immunities: &[],
    V_Effective: &[fire_name, fighting_name, rock_name, steel_name],
    N_Effective: &[ice_name],
};

const FIGHTING: Type = Type {
    type_name: fighting_name,
    Resistance: &[bug_name, rock_name, dark_name],
    Immunities: &[ghost_name],
    V_Effective: &[flying_name, psychic_name, fairy_name],
    N_Effective: &[rock_name, bug_name, dark_name],
};

const POISON: Type = Type {
    type_name: poison_name,
    Resistance: &[fighting_name, poison_name, bug_name, grass_name, fairy_name],
    Immunities: &[steel_name],
    V_Effective: &[ground_name, psychic_name],
    N_Effective: &[fighting_name, poison_name, bug_name, grass_name, fairy_name],
};

const GROUND: Type = Type {
    type_name: ground_name,
    Resistance: &[poison_name, rock_name],
    Immunities: &[electric_name],
    V_Effective: &[water_name, grass_name, ice_name],
    N_Effective: &[poison_name, rock_name],
};

const FLYING: Type = Type {
    type_name: flying_name,
    Resistance: &[fighting_name, bug_name, grass_name],
    Immunities: &[ground_name],
    V_Effective: &[electric_name, ice_name, rock_name],
    N_Effective: &[fighting_name, bug_name, grass_name],
};

const PSYCHIC: Type = Type {
    type_name: psychic_name,
    Resistance: &[fighting_name, psychic_name],
    Immunities: &[],
    V_Effective: &[bug_name, ghost_name, dark_name],
    N_Effective: &[fighting_name, psychic_name],
};

const BUG: Type = Type {
    type_name: bug_name,
    Resistance: &[fighting_name, ground_name, grass_name],
    Immunities: &[],
    V_Effective: &[fire_name, flying_name, rock_name],
    N_Effective: &[fighting_name, ground_name, grass_name],
};

const ROCK: Type = Type {
    type_name: rock_name,
    Resistance: &[normal_name, fire_name, poison_name, flying_name],
    Immunities: &[],
    V_Effective: &[water_name, grass_name, fighting_name, ground_name, steel_name],
    N_Effective: &[normal_name, fire_name, poison_name, flying_name],
};

const GHOST: Type = Type {
    type_name: ghost_name,
    Resistance: &[poison_name, bug_name],
    Immunities: &[normal_name, fighting_name],
    V_Effective: &[ghost_name, dark_name],
    N_Effective: &[poison_name, bug_name],
};

const DRAGON: Type = Type {
    type_name: dragon_name,
    Resistance: &[fire_name, water_name, electric_name, grass_name],
    Immunities: &[],
    V_Effective: &[ice_name, dragon_name, fairy_name],
    N_Effective: &[fire_name, water_name, electric_name, grass_name],
};

const DARK: Type = Type {
    type_name: dark_name,
    Resistance: &[ghost_name, dark_name],
    Immunities: &[psychic_name],
    V_Effective: &[fighting_name, bug_name, fairy_name],
    N_Effective: &[ghost_name, dark_name],
};

const STEEL: Type = Type {
    type_name: steel_name,
    Resistance: &[
        normal_name, grass_name, ice_name, flying_name,
        psychic_name, bug_name, rock_name, dragon_name,
        steel_name, fairy_name
    ],
    Immunities: &[poison_name],
    V_Effective: &[fire_name, fighting_name, ground_name],
    N_Effective: &[
        normal_name, grass_name, ice_name, flying_name,
        psychic_name, bug_name, rock_name, dragon_name,
        steel_name, fairy_name
    ],
};

const FAIRY: Type = Type {
    type_name: fairy_name,
    Resistance: &[fighting_name, bug_name, dark_name],
    Immunities: &[dragon_name],
    V_Effective: &[poison_name, steel_name],
    N_Effective: &[fighting_name, bug_name, dark_name],
};
