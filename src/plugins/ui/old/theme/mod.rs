use bevy::prelude::*;

pub mod button;
pub mod text;
pub use button::*;
pub use text::*;

pub struct ThemePlugin;
impl Plugin for ThemePlugin {
    fn build(&self, _app: &mut App) {
        // todo
    }
}
