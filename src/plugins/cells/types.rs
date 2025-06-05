// edge case: what to do with edges? loop the world or consider outside the grid a set type?
// traditional behavior is loop, but set type could lead to some interesting strategies, for example seeds that activate when placed at the edge

pub type Neighborhood = [CellCondition; 9];

#[derive(Clone, Debug, PartialEq)]
pub struct Seed {
    wake_condition: Vec<Neighborhood>,
    f: fn(Neighborhood) -> f32,
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
