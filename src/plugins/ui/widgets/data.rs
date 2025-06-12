use bevy::{prelude::*, ui::RelativeCursorPosition};

#[derive(Component, Default, Reflect, Debug)]
#[reflect]
pub struct SelectInput {
    // points to the current select node
    pub value: String,
}

#[derive(Component, Debug, Reflect)]
#[reflect]
pub struct SelectOption {
    pub select: Entity,
}

#[derive(Event, Reflect, Debug, Clone)]
#[reflect]
pub struct SelectionChangedEvent {
    pub select: Entity,
    pub option: Entity,
    pub value: String,
}

#[derive(Event, Reflect, Clone, Copy, Debug)]
#[reflect]
pub struct SliderChangedEvent {
    pub slider: Entity,
    pub value: f32,
}

pub const TAG_AXIS: &str = "axis";

#[derive(Default, Reflect)]
#[reflect]
pub enum SliderAxis {
    #[default]
    Horizontal,
    Vertical,
}

impl From<&str> for SliderAxis {
    fn from(value: &str) -> Self {
        match value {
            "y" => SliderAxis::Vertical,
            _ => SliderAxis::Horizontal,
        }
    }
}

/// Slider Component holds the current value
#[derive(Component, Reflect)]
#[reflect]
pub struct Slider {
    pub value: f32,
    pub axis: SliderAxis,
}

/// Slider Nob, which represent the button
#[derive(Component, Reflect)]
#[reflect]
pub struct SliderNob {
    pub slider: Entity,
}

#[derive(Component, Default, Debug)]
#[require(RelativeCursorPosition)]
pub struct Scrollable;
