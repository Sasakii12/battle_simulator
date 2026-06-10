use crate::types::*;

enum Category {
    Physical,
    Special,
}

enum Accuracy {
    Acc(u32),
    Full,
}

struct Move {
    move_type: Type<'static>,
    base_power: u32,
    category: Category,
    accuracy: Accuracy,
    priority: u32,
    pp: u32,
}


const FLOWER_TRICK: Move = Move {
    move_type: GRASS,
    base_power: 70,
    category: Category::Physical,
    accuracy: Accuracy::Full,
    priority: 0,
    pp: 10,
};
