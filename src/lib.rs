pub mod pokemon;
pub mod types;
pub mod moves;
pub mod damage;

use crate::{moves::FLOWER_TRICK, pokemon::Pokemon};

pub fn battle_loop(poke1: Pokemon, poke2: Pokemon) {
    println!("Battle Loop Example: \n");

    println!("Trainer 1 sends out: {}", poke1.base_stats.name);
    println!("Trainer 2 sends out: {}", poke2.base_stats.name);
    println!("");

    println!("{} uses : Flower Trick", poke1.base_stats.name);
    println!("It does {} to ", damage::damage(poke1, poke2.clone(), FLOWER_TRICK)); // Later should
                                                                                    // have whether
                                                                                    // the move was
                                                                                    // effective or
                                                                                    // not
    print!("{} \n", poke2.base_stats.name);

    println!("End battle");
}
