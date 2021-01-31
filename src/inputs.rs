use crate::common::*;

#[derive(Debug)]
pub struct SetupMove {
    pub stars: [Piece; 2],
    pub ship: Piece,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnPhase {
    Started,
    FreeMove(SystemIndex, Color),
    Sacrifice(Color, u8), // number of remaining moves available
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Action {
    pub system: SystemIndex,
    pub ship: Piece,
    pub color_action: ColorAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorAction {
    RedAction(RedActionInput),
    BlueAction(Color),
    GreenAction,
    YellowAction(YellowActionInput),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedActionInput {
    pub enemy_player: PlayerIndex,
    pub ship_to_take: Piece,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YellowActionInput {
    Existing(SystemIndex),
    Discover(Piece),
}

