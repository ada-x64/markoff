use bevy::{color::palettes::tailwind, prelude::*};

#[derive(Bundle, Clone, Debug)]
pub struct TextBundleBase<M: Component + Default> {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
    pub shadow: TextShadow,
    pub marker: M,
}
impl<M: Component + Default> TextBundleBase<M> {
    #[doc(alias = "unfocused")]
    pub fn new(text: impl ToString) -> Self {
        Self::unfocused(text)
    }
    #[doc(alias = "new")]
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
    pub fn title(text: impl ToString) -> Self {
        let mut this = Self::new(text);
        this.font.font_size = 64.;
        this
    }
}
