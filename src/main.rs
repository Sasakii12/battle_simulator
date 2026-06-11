mod pokemon;
mod types;
mod moves;
mod damage;
use pokemon::{PokemonBaseStats, IVSpread, EVSpread};
use types::{TypeName, Type, GRASS, DARK, NORMAL, FIRE};
use moves::*;
use damage::*;

fn main() {
    let meow_base = PokemonBaseStats  {
        name: String::from("Meowscarada"),
        types: GRASS.eval_type(&DARK),
        hp: 76,
        attack: 110,
        def: 70,
        spatk: 81,
        spdef: 70,
        speed: 123,
    };
    let cinder_base = PokemonBaseStats  {
        name: String::from("Cinderace"),
        types: FIRE.eval_type(&NORMAL),
        hp: 80,
        attack: 116,
        def: 75,
        spatk: 65,
        spdef: 75,
        speed: 119,
    };
    let meow_moves = vec![FLOWER_TRICK];
    let meow_iv = IVSpread { hp: 31, attack: 31, def: 31, spatk: 31, spdef:31, speed: 31};
    let meow_ev = EVSpread { hp: 0, attack: 0, def: 0, spatk: 0, spdef: 0, speed: 0};
    let meow = meow_base.evaluate(meow_iv, meow_ev, meow_moves);

    let cinder_moves = vec![PYRO_BALL];
    let cinder_iv = IVSpread { hp: 31, attack: 31, def: 31, spatk: 31, spdef:31, speed: 31};
    let cinder_ev = EVSpread { hp: 0, attack: 0, def: 0, spatk: 0, spdef: 0, speed: 0};
    let cinder = cinder_base.evaluate(cinder_iv, cinder_ev, cinder_moves);
    println!("Hello, world!");
    println!("{:?}", meow.base_stats.types);
    println!("{:?}", cinder.base_stats.types);
    println!("{:?}", meow.moves[0].move_type.type_name);
    println!("{}", damage::damage(meow, cinder, moves::FLOWER_TRICK));
}
