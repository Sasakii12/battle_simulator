mod pokemon;
use pokemon::{PokemonBaseStats, IVSpread};

fn main() {
    let meow_base = PokemonBaseStats  {
        name: String::from("Meowscarada"),
        types: vec![String::from("Grass"), String::from("Dark")],
        hp: 76,
        attack: 110,
        def: 70,
        spatk: 81,
        spdef: 70,
        speed: 123,
    };
    let meow_iv = IVSpread { hp: 31, attack: 31, def: 31, spatk: 31, spdef:31, speed: 31};
    println!("Hello, world!");
}
