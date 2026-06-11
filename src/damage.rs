use crate::pokemon::{Pokemon};
use crate::moves::{Move};

pub fn damage(attacker: Pokemon, defender: Pokemon, attacking_move: Move) -> i32 {
    if defender.base_stats.types.immunity.contains(&attacking_move.move_type.type_name) {
        return 0;
    }
    let mut stab = 1.;
    if attacker.base_stats.types.type_name.0 == attacking_move.move_type.type_name || attacker.base_stats.types.type_name.1 == attacking_move.move_type.type_name {
        stab = 1.5;
    }
    todo!();
}

pub fn battle_loop() {}
