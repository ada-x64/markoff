use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

use crate::{
    screens::{Screen, ScreenMarker},
    theme::text::TextBundle,
    widgets::button,
};

#[hot]
pub fn init(mut commands: Commands) {
    commands
        .spawn((
            ScreenMarker,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // children![title, buttons_wrapper],
        ))
        .with_children(|spawner| {
            spawner.spawn((
                Node {
                    height: Val::Percent(33.),
                    ..Default::default()
                },
                children![TextBundle::title("MARKOFF!")],
            ));
            let buttons_wrapper = (Node {
                width: Val::Percent(33.),
                height: Val::Percent(33.),
                margin: UiRect::top(Val::Px(32.)).with_bottom(Val::Px(32.)),
                ..Default::default()
            },);
            spawner.spawn(buttons_wrapper).with_children(|spawner| {
                spawner.spawn(button("play", "New Game")).observe(on_click);
                spawner
                    .spawn(button("sandbox", "Sandbox"))
                    .observe(on_click);
                #[cfg(not(target_arch = "wasm32"))]
                spawner.spawn(button("quit", "Quit")).observe(on_click);
            });
        });
}

pub fn on_click(
    trigger: Trigger<Pointer<Click>>,
    query: Query<&Name, With<Button>>,
    mut commands: Commands,
    mut screen: ResMut<NextState<Screen>>,
) {
    if let Ok(btn) = query.get(trigger.target()) {
        match btn.as_str() {
            "play" => screen.set(Screen::GameSettings),
            "sandbox" => screen.set(Screen::Sandbox),
            "quit" => {
                commands.send_event(AppExit::Success);
            }
            _ => {
                warn!("Got unknown button press: '{btn}'")
            }
        }
    }
}
