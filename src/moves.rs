use crate::types::Type;

enum Category {
    Physical,
    Special,
}

struct Move {
    move_type: Type<'static>,
    base_power: u32,
    category: Category,
    accuracy: u32,
    priority: u32,
    pp: u32,
}
