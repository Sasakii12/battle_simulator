pub struct PokemonBaseStats {
    pub name: String,
    pub types: Vec<String>,
    pub hp: u32,
    pub attack: u32,
    pub def: u32,
    pub spdef: u32,
    pub spatk: u32,
    pub speed: u32,
}

pub struct Pokemon {
    pub base_stats: PokemonBaseStats,
    pub nature: String,
}

pub struct IVSpread {
    pub hp: u32,
    pub attack: u32,
    pub def: u32,
    pub spdef: u32,
    pub spatk: u32,
    pub speed: u32,
}

pub struct EVSpread {
    pub hp: u32,
    pub attack: u32,
    pub def: u32,
    pub spdef: u32,
    pub spatk: u32,
    pub speed: u32,
}

    fn stat(stat: u32, iv: u32, ev: u32) -> u32 {
        ((2 *  stat + iv + (ev as f32 / 4.).floor() as u32) + 5) * 1
    }

impl PokemonBaseStats {
    pub fn evaluate(self, iv : IVSpread, ev: EVSpread) -> Pokemon {
        let hp = (((2 * self.hp + iv.hp + ((ev.hp as f32 / 4.).floor() as u32)) as f32 * 100.) / 100.).floor() as u32 + 100 + 10;
        let def = stat(self.def, iv.def, ev.def);
        let attack = stat(self.attack, iv.attack, ev.attack);
        let spatk = stat(self.spatk, iv.spatk, ev.spatk);
        let spdef = stat(self.spdef, iv.spdef, ev.spdef);
        let speed = stat(self.speed, iv.speed, ev.speed);

        Pokemon {
                base_stats: PokemonBaseStats{name: self.name, types: self.types, hp,attack,def,spatk,spdef,speed},
                nature: String::from("e")
        }
    } 
}
