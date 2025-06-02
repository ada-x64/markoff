use bevy::prelude::*;

pub mod types;
pub mod prelude {
    pub use super::types::*;
}

pub struct TeamsPlugin {}
impl Plugin for TeamsPlugin {
    fn build(&self, app: &mut App) {}
}
