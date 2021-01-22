use crate::common::*;

#[derive(Debug)]
pub struct SetupMove {
    pub player: PlayerIndex,
    pub stars: [Piece; 2],
    pub ship: Piece,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclareCatastrophe {
    pub system: SystemIndex,
    pub color: Color,
}

#[derive(Debug)]
pub enum TurnInput {
    Free(FreeMove),
    Sacrifice(SacrificeMove),
}

#[derive(Debug)]
pub struct FreeMove {
    pub system: SystemIndex,
    pub color: Color,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Catastrophe(DeclareCatastrophe),
    RedAction(RedActionInput),
    //TODO all the other colors
}

#[derive(Debug)]
pub struct SacrificeMove {
    //TODO
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedActionInput {
    pub system: SystemIndex,
    pub ship: Piece,
    pub enemy_player: PlayerIndex,
    pub ship_to_take: Piece,
}

