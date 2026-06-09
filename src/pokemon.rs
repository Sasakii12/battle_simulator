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

struct Pokemon {
    base_stats: PokemonBaseStats,
    nature: String,
}

pub struct IVSpread {
    pub hp: u32,
    pub attack: u32,
    pub def: u32,
    pub spdef: u32,
    pub spatk: u32,
    pub speed: u32,
}

struct EVSpread {
    hp: u32,
    attack: u32,
    def: u32,
    spdef: u32,
    spatk: u32,
    speed: u32,
}

impl PokemonBaseStats {
    fn evaluate(iv : IVSpread, ev: EVSpread) {} 
}
