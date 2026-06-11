use crate::types::*;

pub enum Category {
    Physical,
    Special,
}

pub enum Accuracy {
    Acc(f32),
    Full,
}

pub struct Move {
    pub move_type: Type<'static>,
    pub base_power: u32,
    pub category: Category,
    pub accuracy: Accuracy,
    pub priority: u32,
    pub pp: u32,
    pub always_crit: bool,
}


pub const FLOWER_TRICK: Move = Move {
    move_type: GRASS,
    base_power: 70,
    category: Category::Physical,
    accuracy: Accuracy::Full,
    priority: 0,
    pp: 10,
    always_crit: true,
};

const HYPER_VOICE: Move = Move {
    move_type: NORMAL,
    base_power: 90,
    category: Category::Special,
    accuracy: Accuracy::Acc(100.),
    priority: 0,
    pp: 10,
    always_crit: false,
};
const PYRO_BALL: Move = Move {
    move_type: FIRE,
    base_power: 120,
    category: Category::Physical,
    accuracy: Accuracy::Acc(90.),
    priority: 0,
    pp: 5,
    always_crit: false,
};
