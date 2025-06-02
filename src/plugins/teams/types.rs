use crate::cells::types::*;
use bevy::prelude::*;

pub type TeamID = u8;
pub type PlayerID = u8;
pub struct Team {
    id: TeamID,
    name: String,
    players: Vec<Player>,
    color: Color,
}
pub struct Player {
    id: PlayerID,
    name: String,
    seeds: Vec<Seed>,
}
