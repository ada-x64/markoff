use bevy::prelude::*;
pub mod button;
pub use button::*;

pub struct WidgetsPlugin;
impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        let _ = { app.add_systems(Update, button::update) };
    }
}
