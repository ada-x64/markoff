use bevy::prelude::*;
pub mod button;
pub use button::*;
pub mod grid;
pub use grid::*;
pub mod text;
pub use text::*;
pub mod bg;
pub use bg::*;

pub struct WidgetsPlugin;
impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        let _ = { app.add_systems(Update, button::update) };
    }
}
