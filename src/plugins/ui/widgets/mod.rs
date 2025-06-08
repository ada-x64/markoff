use bevy::prelude::*;

pub mod slider;
use slider::SliderWidgetPlugin;
pub mod select;
use select::SelectWidgetPlugin;
pub mod stamp_selector;
use stamp_selector::StampSelectorWidgetPlugin;

pub struct WidgetsPlugin;
impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SliderWidgetPlugin,
            SelectWidgetPlugin,
            StampSelectorWidgetPlugin,
        ));
    }
}
