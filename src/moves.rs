use crate::types::*;

enum Category {
    Physical,
    Special,
}

enum Accuracy {
    Acc(f32),
    Full,
}

struct Move {
    move_type: Type<'static>,
    base_power: u32,
    category: Category,
    accuracy: Accuracy,
    priority: u32,
    pp: u32,
    always_crit: bool,
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
