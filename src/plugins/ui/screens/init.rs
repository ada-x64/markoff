use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_hui::prelude::*;
use tiny_bail::prelude::*;

use crate::ui::{data::ScreenRoot, screens::CurrentScreen};

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct SplashTimer(Timer);

#[cfg(feature = "dev")]
const SPLASH_SECS: f32 = 0.;
#[cfg(not(feature = "dev"))]
const SPLASH_SECS: f32 = 2.;

pub struct InitScreenPlugin;
impl Plugin for InitScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SplashTimer(Timer::from_seconds(
            SPLASH_SECS,
            TimerMode::Once,
        )))
        .add_systems(OnEnter(CurrentScreen::Init), render)
        .add_systems(
            Update,
            (update, tick_fade_in_out, apply_fade_in_out).run_if(in_state(CurrentScreen::Init)),
        );
    }
}

fn render(mut commands: Commands, server: ResMut<AssetServer>) {
    let image = server.load_with_settings(
        "textures/bevy_splash.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::linear();
        },
    );
    commands.spawn((
        ScreenRoot,
        Node {
            width: Val::Vw(100.),
            height: Val::Vh(100.),
            ..Default::default()
        },
        children![(
            Node {
                margin: UiRect::all(Val::Auto),
                width: Val::Percent(70.),
                ..Default::default()
            },
            ImageNode {
                image,
                ..Default::default()
            },
            ImageNodeFadeInOut {
                total_duration: SPLASH_SECS,
                fade_duration: 0.5,
                t: 0.,
            }
        )],
    ));
}

fn update(
    mut timer: ResMut<SplashTimer>,
    time: Res<Time<Real>>,
    auto_load: Res<State<AutoLoadState>>,
    mut next_state: ResMut<NextState<CurrentScreen>>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    timer.tick(time.delta());
    let handle = r!(server
        .get_handle::<HtmlTemplate>("hui/screens/main_menu.xml")
        .ok_or("No handle to main_menu.xml"));
    if timer.finished()
        && matches!(**auto_load, AutoLoadState::Finished)
        && server.is_loaded(&handle)
    {
        next_state.set(CurrentScreen::MainMenu);
        commands.remove_resource::<SplashTimer>();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ImageNodeFadeInOut {
    /// Total duration in seconds.
    total_duration: f32,
    /// Fade duration in seconds.
    fade_duration: f32,
    /// Current progress in seconds, between 0 and [`Self::total_duration`].
    t: f32,
}

impl ImageNodeFadeInOut {
    fn alpha(&self) -> f32 {
        // Normalize by duration.
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;

        // Regular trapezoid-shaped graph, flat at the top with alpha = 1.0.
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }
}

fn tick_fade_in_out(time: Res<Time<Real>>, mut animation_query: Query<&mut ImageNodeFadeInOut>) {
    for mut anim in &mut animation_query {
        anim.t += time.delta_secs();
    }
}

fn apply_fade_in_out(mut animation_query: Query<(&ImageNodeFadeInOut, &mut ImageNode)>) {
    for (anim, mut image) in &mut animation_query {
        image.color.set_alpha(anim.alpha())
    }
}
