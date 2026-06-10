mod pokemon;
mod types;
mod moves;
use pokemon::{PokemonBaseStats, IVSpread, EVSpread};
use types::{TypeName, Type, GRASS, DARK};

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
    let meow_iv = IVSpread { hp: 31, attack: 31, def: 31, spatk: 31, spdef:31, speed: 31};
    let meow_ev = EVSpread { hp: 0, attack: 0, def: 0, spatk: 0, spdef: 0, speed: 0};
    let meow = meow_base.evaluate(meow_iv, meow_ev);
    println!("Hello, world!");
    println!("{:?}", meow.base_stats.types);
}
