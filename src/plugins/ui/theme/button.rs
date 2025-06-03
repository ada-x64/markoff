use bevy::{color::palettes::tailwind, prelude::*};

use crate::theme::text::TextBundleBase;

/// Marker component
#[derive(Debug, Clone, Copy, Component)]
pub struct ButtonStyle;

/// Button styles. Should be parent to ButtonTextBundle.
#[derive(Debug, Clone, Bundle)]
pub struct ButtonStyleBundle {
    pub border_color: BorderColor,
    pub radius: BorderRadius,
    pub background: BackgroundColor,
    pub color: TextColor,
    pub node: Node,
    marker: ButtonStyle,
}
impl Default for ButtonStyleBundle {
    #[doc(alias = "unfocused")]
    fn default() -> Self {
        Self {
            marker: ButtonStyle,
            border_color: BorderColor(tailwind::SLATE_950.into()),
            radius: BorderRadius::px(2., 2., 2., 2.),
            background: BackgroundColor(tailwind::SLATE_700.into()),
            color: TextColor(tailwind::SLATE_50.into()),
            node: Node {
                width: Val::Auto,
                height: Val::Auto,
                border: UiRect::all(Val::Px(2.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        }
    }
}
impl ButtonStyleBundle {
    #[doc(alias = "default")]
    pub fn unfocused() -> Self {
        Self::default()
    }
    pub fn hovered() -> Self {
        Self {
            background: BackgroundColor(tailwind::SLATE_600.into()),
            border_color: BorderColor(tailwind::AMBER_600.into()),
            ..Default::default()
        }
    }
    pub fn pressed() -> Self {
        Self {
            background: BackgroundColor(tailwind::SLATE_800.into()),
            border_color: BorderColor(tailwind::AMBER_400.into()),
            ..Default::default()
        }
    }
}

/// Marker component.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ButtonText;
pub type ButtonTextBundle = TextBundleBase<ButtonText>;
