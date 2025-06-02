use crate::theme::button::*;
use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;
use std::borrow::Cow;

pub fn button(name: impl Into<Cow<'static, str>>, text: impl ToString) -> impl Bundle {
    (
        Name::new(name),
        Button,
        ButtonStyleBundle::unfocused(),
        children![ButtonTextBundle::unfocused(text)],
    )
}

pub fn update(
    mut style_q: Query<
        (
            &Interaction,
            &mut BorderColor,
            &mut BorderRadius,
            &mut BackgroundColor,
            &Children,
            &mut Node,
        ),
        (Changed<Interaction>, With<ButtonStyle>),
    >,
    mut text_q: Query<(&Text, &mut TextFont, &mut TextColor, &mut TextShadow), With<ButtonText>>,
) {
    info_once!("Update!");
    for (
        interaction,
        mut border_color,
        mut border_radius,
        mut background_color,
        children,
        mut node,
    ) in &mut style_q
    {
        let child = children.first().unwrap();
        let (text, mut font, mut text_color, mut text_shadow) = text_q.get_mut(*child).unwrap();
        let (style, text_style) = match interaction {
            Interaction::None => (
                ButtonStyleBundle::unfocused(),
                ButtonTextBundle::unfocused(text.to_string()),
            ),
            Interaction::Pressed => (
                ButtonStyleBundle::pressed(),
                ButtonTextBundle::pressed(text.to_string()),
            ),
            Interaction::Hovered => (
                ButtonStyleBundle::hovered(),
                ButtonTextBundle::hovered(text.to_string()),
            ),
        };
        *border_color = style.border_color;
        *border_radius = style.radius;
        *background_color = style.background;
        *node = style.node;
        *font = text_style.font;
        *text_color = text_style.color;
        *text_shadow = text_style.shadow;
    }
}
