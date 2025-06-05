use crate::ui::theme::text::TextBundleBase;
use bevy::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct TextBundleMarker;
pub type TextBundle = TextBundleBase<TextBundleMarker>;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ButtonText;
pub type ButtonTextBundle = TextBundleBase<ButtonText>;
