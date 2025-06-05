use crate::cells::types::*;
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
    pub seeds: Vec<Seed>,
}
