use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    NULL,
}

#[derive(Clone, Copy)]
pub struct Type<'a> {
    pub type_name: TypeName,

    pub resistances: &'a [TypeName],
    pub immunities: &'a [TypeName],
    pub weaknesses: &'a [TypeName],

    pub v_effective: &'a [TypeName],
    pub n_effective: &'a [TypeName],
    pub no_effect: &'a [TypeName],
}

#[derive(Debug)]
pub struct DualType {
    pub type_name: (TypeName, TypeName),

    // Defensive
    pub quad_weakness: Vec<TypeName>,
    pub weakness: Vec<TypeName>,
    pub resistance: Vec<TypeName>,
    pub quad_resistance: Vec<TypeName>,
    pub immunity: Vec<TypeName>,
}

impl<'a> Type<'a> {
    pub fn eval_type(&self, second_type: &Type) -> DualType {
        let type1_resist: HashSet<TypeName> = self.resistances.iter().cloned().collect();
        let type1_immun: HashSet<TypeName> = self.immunities.iter().cloned().collect();
        let type1_weak: HashSet<TypeName> = self.weaknesses.iter().cloned().collect();

        let type2_resist: HashSet<TypeName> = second_type.resistances.iter().cloned().collect();
        let type2_immun: HashSet<TypeName> = second_type.immunities.iter().cloned().collect();
        let type2_weak: HashSet<TypeName> = second_type.weaknesses.iter().cloned().collect();

        let all_types = [
            TypeName::Normal, TypeName::Fire, TypeName::Water, TypeName::Electric,
            TypeName::Grass, TypeName::Ice, TypeName::Fighting, TypeName::Poison,
            TypeName::Ground, TypeName::Flying, TypeName::Psychic, TypeName::Bug,
            TypeName::Rock, TypeName::Ghost, TypeName::Dragon, TypeName::Dark,
            TypeName::Steel, TypeName::Fairy,
        ];

        let mut quad_weakness = Vec::new();
        let mut weakness = Vec::new();
        let mut resistance = Vec::new();
        let mut quad_resistance = Vec::new();
        let mut immunity = Vec::new();

        for &atk in &all_types {
            // Immunity overrides everything
            if type1_immun.contains(&atk) || type2_immun.contains(&atk) {
                immunity.push(atk);
                continue;
            }

            let mut multiplier = 1.0_f32;

            if type1_weak.contains(&atk)   { multiplier *= 2.0; }
            if type2_weak.contains(&atk)   { multiplier *= 2.0; }
            if type1_resist.contains(&atk) { multiplier *= 0.5; }
            if type2_resist.contains(&atk) { multiplier *= 0.5; }

            // Use epsilon comparison for floats
            if (multiplier - 4.0).abs() < f32::EPSILON {
                quad_weakness.push(atk);
            } else if (multiplier - 2.0).abs() < f32::EPSILON {
                weakness.push(atk);
            } else if (multiplier - 0.5).abs() < f32::EPSILON {
                resistance.push(atk);
            } else if (multiplier - 0.25).abs() < f32::EPSILON {
                quad_resistance.push(atk);
            }
            // 1.0 = neutral, skip
        }

        DualType {
            type_name: (self.type_name, second_type.type_name),
            quad_weakness,
            weakness,
            resistance,
            quad_resistance,
            immunity,
        }
    }
}

// ── Type chart data ──────────────────────────────────────────────────────────
// Source: Gen 6+ type chart (Fairy added, Steel lost Ghost/Dark resistances)

pub static NORMAL: Type<'static> = Type {
    type_name: TypeName::Normal,
    weaknesses:  &[TypeName::Fighting],
    resistances: &[],
    immunities:  &[TypeName::Ghost],
    v_effective: &[],
    n_effective: &[TypeName::Rock, TypeName::Steel],
    no_effect:   &[TypeName::Ghost],
};

pub static FIRE: Type<'static> = Type {
    type_name: TypeName::Fire,
    weaknesses:  &[TypeName::Water, TypeName::Ground, TypeName::Rock],
    resistances: &[TypeName::Fire, TypeName::Grass, TypeName::Ice, TypeName::Bug, TypeName::Steel, TypeName::Fairy],
    immunities:  &[],
    v_effective: &[TypeName::Grass, TypeName::Ice, TypeName::Bug, TypeName::Steel],
    n_effective: &[TypeName::Fire, TypeName::Water, TypeName::Rock, TypeName::Dragon],
    no_effect:   &[],
};

pub static WATER: Type<'static> = Type {
    type_name: TypeName::Water,
    weaknesses:  &[TypeName::Electric, TypeName::Grass],
    resistances: &[TypeName::Fire, TypeName::Water, TypeName::Ice, TypeName::Steel],
    immunities:  &[],
    v_effective: &[TypeName::Fire, TypeName::Ground, TypeName::Rock],
    n_effective: &[TypeName::Water, TypeName::Grass, TypeName::Dragon],
    no_effect:   &[],
};

pub static ELECTRIC: Type<'static> = Type {
    type_name: TypeName::Electric,
    weaknesses:  &[TypeName::Ground],
    resistances: &[TypeName::Electric, TypeName::Flying, TypeName::Steel],
    immunities:  &[],
    v_effective: &[TypeName::Water, TypeName::Flying],
    n_effective: &[TypeName::Electric, TypeName::Grass, TypeName::Dragon],
    no_effect:   &[TypeName::Ground],
};

pub static GRASS: Type<'static> = Type {
    type_name: TypeName::Grass,
    weaknesses:  &[TypeName::Fire, TypeName::Ice, TypeName::Poison, TypeName::Flying, TypeName::Bug],
    resistances: &[TypeName::Water, TypeName::Electric, TypeName::Grass, TypeName::Ground],
    immunities:  &[],
    v_effective: &[TypeName::Water, TypeName::Ground, TypeName::Rock],
    n_effective: &[TypeName::Fire, TypeName::Grass, TypeName::Poison, TypeName::Flying, TypeName::Bug, TypeName::Dragon, TypeName::Steel],
    no_effect:   &[],
};

pub static ICE: Type<'static> = Type {
    type_name: TypeName::Ice,
    weaknesses:  &[TypeName::Fire, TypeName::Fighting, TypeName::Rock, TypeName::Steel],
    resistances: &[TypeName::Ice],
    immunities:  &[],
    v_effective: &[TypeName::Grass, TypeName::Ground, TypeName::Flying, TypeName::Dragon],
    n_effective: &[TypeName::Fire, TypeName::Water, TypeName::Ice, TypeName::Steel],
    no_effect:   &[],
};

pub static FIGHTING: Type<'static> = Type {
    type_name: TypeName::Fighting,
    weaknesses:  &[TypeName::Flying, TypeName::Psychic, TypeName::Fairy],
    resistances: &[TypeName::Bug, TypeName::Rock, TypeName::Dark],
    immunities:  &[],
    v_effective: &[TypeName::Normal, TypeName::Ice, TypeName::Rock, TypeName::Dark, TypeName::Steel],
    n_effective: &[TypeName::Poison, TypeName::Bug, TypeName::Psychic, TypeName::Flying, TypeName::Fairy],
    no_effect:   &[TypeName::Ghost],
};

pub static POISON: Type<'static> = Type {
    type_name: TypeName::Poison,
    weaknesses:  &[TypeName::Ground, TypeName::Psychic],
    resistances: &[TypeName::Grass, TypeName::Fighting, TypeName::Poison, TypeName::Bug, TypeName::Fairy],
    immunities:  &[],
    v_effective: &[TypeName::Grass, TypeName::Fairy],
    n_effective: &[TypeName::Poison, TypeName::Ground, TypeName::Rock, TypeName::Ghost],
    no_effect:   &[TypeName::Steel],
};

pub static GROUND: Type<'static> = Type {
    type_name: TypeName::Ground,
    weaknesses:  &[TypeName::Water, TypeName::Grass, TypeName::Ice],
    resistances: &[TypeName::Poison, TypeName::Rock],
    immunities:  &[TypeName::Electric],
    v_effective: &[TypeName::Fire, TypeName::Electric, TypeName::Poison, TypeName::Rock, TypeName::Steel],
    n_effective: &[TypeName::Grass, TypeName::Bug],
    no_effect:   &[TypeName::Flying],
};

pub static FLYING: Type<'static> = Type {
    type_name: TypeName::Flying,
    weaknesses:  &[TypeName::Electric, TypeName::Ice, TypeName::Rock],
    resistances: &[TypeName::Grass, TypeName::Fighting, TypeName::Bug],
    immunities:  &[TypeName::Ground],
    v_effective: &[TypeName::Grass, TypeName::Fighting, TypeName::Bug],
    n_effective: &[TypeName::Electric, TypeName::Rock, TypeName::Steel],
    no_effect:   &[],
};

pub static PSYCHIC: Type<'static> = Type {
    type_name: TypeName::Psychic,
    weaknesses:  &[TypeName::Bug, TypeName::Ghost, TypeName::Dark],
    resistances: &[TypeName::Fighting, TypeName::Psychic],
    immunities:  &[],
    v_effective: &[TypeName::Fighting, TypeName::Poison],
    n_effective: &[TypeName::Psychic, TypeName::Steel],
    no_effect:   &[TypeName::Dark],
};

pub static BUG: Type<'static> = Type {
    type_name: TypeName::Bug,
    weaknesses:  &[TypeName::Fire, TypeName::Flying, TypeName::Rock],
    resistances: &[TypeName::Grass, TypeName::Fighting, TypeName::Ground],
    immunities:  &[],
    v_effective: &[TypeName::Grass, TypeName::Psychic, TypeName::Dark],
    n_effective: &[TypeName::Fire, TypeName::Fighting, TypeName::Poison, TypeName::Flying, TypeName::Ghost, TypeName::Steel, TypeName::Fairy],
    no_effect:   &[],
};

pub static ROCK: Type<'static> = Type {
    type_name: TypeName::Rock,
    weaknesses:  &[TypeName::Water, TypeName::Grass, TypeName::Fighting, TypeName::Ground, TypeName::Steel],
    resistances: &[TypeName::Normal, TypeName::Fire, TypeName::Poison, TypeName::Flying],
    immunities:  &[],
    v_effective: &[TypeName::Fire, TypeName::Ice, TypeName::Flying, TypeName::Bug],
    n_effective: &[TypeName::Fighting, TypeName::Ground, TypeName::Steel],
    no_effect:   &[],
};

pub static GHOST: Type<'static> = Type {
    type_name: TypeName::Ghost,
    weaknesses:  &[TypeName::Ghost, TypeName::Dark],
    resistances: &[TypeName::Poison, TypeName::Bug],
    immunities:  &[TypeName::Normal, TypeName::Fighting],
    v_effective: &[TypeName::Psychic, TypeName::Ghost],
    n_effective: &[TypeName::Dark],
    no_effect:   &[TypeName::Normal],
};

pub static DRAGON: Type<'static> = Type {
    type_name: TypeName::Dragon,
    weaknesses:  &[TypeName::Ice, TypeName::Dragon, TypeName::Fairy],
    resistances: &[TypeName::Fire, TypeName::Water, TypeName::Electric, TypeName::Grass],
    immunities:  &[],
    v_effective: &[TypeName::Dragon],
    n_effective: &[TypeName::Steel],
    no_effect:   &[TypeName::Fairy],
};

pub static DARK: Type<'static> = Type {
    type_name: TypeName::Dark,
    weaknesses:  &[TypeName::Fighting, TypeName::Bug, TypeName::Fairy],
    resistances: &[TypeName::Ghost, TypeName::Dark],
    immunities:  &[TypeName::Psychic],
    v_effective: &[TypeName::Psychic, TypeName::Ghost],
    n_effective: &[TypeName::Fighting, TypeName::Dark, TypeName::Fairy],
    no_effect:   &[],
};

pub static STEEL: Type<'static> = Type {
    type_name: TypeName::Steel,
    weaknesses:  &[TypeName::Fire, TypeName::Fighting, TypeName::Ground],
    resistances: &[
        TypeName::Normal, TypeName::Grass, TypeName::Ice, TypeName::Flying,
        TypeName::Psychic, TypeName::Bug, TypeName::Rock, TypeName::Dragon,
        TypeName::Steel, TypeName::Fairy,
    ],
    immunities:  &[TypeName::Poison],
    v_effective: &[TypeName::Ice, TypeName::Rock, TypeName::Fairy],
    n_effective: &[TypeName::Fire, TypeName::Water, TypeName::Electric, TypeName::Steel],
    no_effect:   &[],
};

pub static FAIRY: Type<'static> = Type {
    type_name: TypeName::Fairy,
    weaknesses:  &[TypeName::Poison, TypeName::Steel],
    resistances: &[TypeName::Fighting, TypeName::Bug, TypeName::Dark],
    immunities:  &[TypeName::Dragon],
    v_effective: &[TypeName::Fighting, TypeName::Dragon, TypeName::Dark],
    n_effective: &[TypeName::Fire, TypeName::Poison, TypeName::Steel],
    no_effect:   &[],
};
