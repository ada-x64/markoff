use bevy::{color::palettes::tailwind, prelude::*};

/// Button text styles.
#[derive(Bundle, Clone, Debug)]
pub struct TextBundle<M: Component + Default> {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
    pub shadow: TextShadow,
    marker: M,
}
impl<M: Component + Default> TextBundle<M> {
    pub fn unfocused(text: impl ToString) -> Self {
        Self {
            text: Text(text.to_string()),
            font: TextFont::default(),
            color: TextColor(tailwind::SLATE_100.into()),
            shadow: TextShadow {
                color: Color::srgba(0., 0., 0., 0.),
                ..Default::default()
            },
            marker: M::default(),
        }
    }
    pub fn hovered(text: impl ToString) -> Self {
        Self {
            color: TextColor(tailwind::SLATE_50.into()),
            ..Self::unfocused(text)
        }
    }
    pub fn pressed(text: impl ToString) -> Self {
        Self {
            color: TextColor(tailwind::SLATE_100.into()),
            ..Self::unfocused(text)
        }
    }
}
