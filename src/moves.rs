use crate::types::*;

pub enum Category {
    Physical,
    Special,
}

enum Accuracy {
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


const FLOWER_TRICK: Move = Move {
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
