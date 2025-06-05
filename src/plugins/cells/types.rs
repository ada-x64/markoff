// edge case: what to do with edges? loop the world or consider outside the grid a set type?
// traditional behavior is loop, but set type could lead to some interesting strategies, for example seeds that activate when placed at the edge

use crate::teams::types::*;

pub type Neighborhood = [CellCondition; 9];

pub struct Seed {
    wake_condition: Vec<Neighborhood>,
    f: fn(Neighborhood) -> f32,
}

pub struct Grid {
    state: Vec<CellState>,
    width: u32,
    height: u32,
}

/// Stored in the grid
pub enum CellState {
    Empty,
    Active,
    Seed(Seed, TeamID),
    Captured(TeamID),
}

/// Used to check cell state
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub enum CellCondition {
    #[default]
    Empty,
    Active,
    Owned,
    Enemy,
}
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CellResult {
    Empty,
    Active,
    Untouched,
}
impl CellCondition {
    pub fn from_cell_state(s: CellState, team: u8) -> Self {
        match s {
            CellState::Empty => Self::Empty,
            CellState::Active => Self::Active,
            CellState::Captured(t) if t == team => Self::Owned,
            _ => Self::Enemy,
        }
    }
}
