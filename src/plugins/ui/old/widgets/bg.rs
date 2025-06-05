use bevy::prelude::*;

pub fn bg(image: Handle<Image>) -> impl Bundle {
    (
        Sprite {
            image,
            ..Default::default()
        },
        ZIndex(-1),
    )
}
