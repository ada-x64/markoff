use bevy::{color::palettes::tailwind, prelude::*};

// TODO: theme

pub fn grid_layout(
    grid_template_rows: Vec<RepeatedGridTrack>,
    grid_template_columns: Vec<RepeatedGridTrack>,
) -> impl Bundle {
    Node {
        display: Display::Grid,
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        grid_template_rows,
        grid_template_columns,
        ..Default::default()
    }
}
pub fn grid_panel(
    grid_row: GridPlacement,
    grid_column: GridPlacement,
    border: UiRect,
) -> impl Bundle {
    (
        Node {
            grid_row,
            grid_column,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            border,
            ..Default::default()
        },
        BackgroundColor(tailwind::GRAY_800.into()),
        BorderColor(tailwind::GRAY_950.into()),
    )
}
pub fn grid_bar(
    grid_row: GridPlacement,
    grid_column: GridPlacement,
    border: UiRect,
) -> impl Bundle {
    (
        Node {
            grid_row,
            grid_column,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            border,
            ..Default::default()
        },
        BackgroundColor(tailwind::GRAY_800.into()),
        BorderColor(tailwind::GRAY_950.into()),
    )
}
