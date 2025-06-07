use bevy::prelude::*;

pub type TeamID = u8;
pub type PlayerID = u8;

#[derive(Clone, Debug, PartialEq)]
pub struct Team {
    pub id: TeamID,
    pub name: String,
    pub players: Vec<PlayerID>,
    pub color: [u8; 4],
}
#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub id: PlayerID,
    pub name: String,
}

/// Used to check cell state
#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub enum CellCondition {
    #[default]
    Empty,
    Active,
    Owned,
    Enemy,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CellResult {
    Empty,
    Active,
    Untouched,
}
