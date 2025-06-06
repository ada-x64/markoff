use bevy::prelude::*;

use crate::ui::widgets::slider::SliderWidgetPlugin;

pub mod slider;

pub struct WidgetsPlugin;
impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SliderWidgetPlugin);
    }
}
