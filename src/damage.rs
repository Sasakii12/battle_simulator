use crate::pokemon::{Pokemon};
use crate::moves::{Move, Category};
use rand::prelude::*;

pub fn damage(attacker: Pokemon, defender: Pokemon, attacking_move: Move) -> i32 {
    if defender.base_stats.types.immunity.contains(&attacking_move.move_type.type_name) {
        return 0;
    }

    let mut effective = 1.;

    if defender.base_stats.types.quad_weakness.contains(&attacking_move.move_type.type_name) {
        effective = 4.;
    } else if defender.base_stats.types.weakness.contains(&attacking_move.move_type.type_name) {
        effective = 2.;
    } else if defender.base_stats.types.resistance.contains(&attacking_move.move_type.type_name) {
        effective = 0.5;
    } else if defender.base_stats.types.quad_resistance.contains(&attacking_move.move_type.type_name) {
        effective = 0.25;
    }

    let mut stab = 1.;

    if attacker.base_stats.types.type_name.0 == attacking_move.move_type.type_name || attacker.base_stats.types.type_name.1 == attacking_move.move_type.type_name {
        stab = 1.5;
    } 

    let mut rng = rand::rng();

    let mut crit = 1.;

    if attacking_move.always_crit {
        crit = 2.;
    } else if rng.random_range(1..=100) < 6 {
        crit = 2.;
    } 
    
    let attacking_stat;
    let defending_stat;

    match attacking_move.category {
        Category::Special => {
            attacking_stat = attacker.base_stats.spatk;
            defending_stat = defender.base_stats.spdef;
        },
        Category::Physical => {
            attacking_stat = attacker.base_stats.attack;
            defending_stat = defender.base_stats.def;
        }
    }

    let rand_dam = rng.random_range(85..=100) as f32 / 100.;

    let dam = (((2 * 100) / 5 + 2) * attacking_move.base_power * (attacking_stat / defending_stat) / 50 + 2) as f32 * stab * crit * effective * rand_dam;
    let round = dam.round() as i32;
    round
}

pub fn battle_loop() {}
