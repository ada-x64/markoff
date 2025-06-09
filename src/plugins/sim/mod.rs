use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
};
use derivative::Derivative;

use types::*;

use crate::{
    sim::web::SoftwareSimSet,
    stamps::{Stamp, Stamps},
    ui::widgets::sim_image::SimImageNode,
};
// TODO: This is crashing!
// But we have bigger fish to fry right now.
// mod native;
mod types;
mod web;

pub type PixelColor<'a> = &'a [u8; 4];
pub const BLACK: PixelColor = &[0, 0, 0, 255];
pub const WHITE: PixelColor = &[255, 255, 255, 255];

#[derive(Event)]
pub struct StampEvent;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SimState {
    #[default]
    Closed,
    Init,
    Paused,
    Running,
}

#[derive(Resource, Debug, Default)]
pub struct SimGameplayState {
    pub current_stamp: Option<String>,
    pub num_steps: u32,
}

// This should probably be a component
// #[derive(Default, Resource, ExtractResource, PartialEq, Eq, Hash, Copy, Clone)]
// pub enum SimRenderState {
//     #[default]
//     Closed,
//     Init,
//     Running,
//     Paused,
// }
// impl From<SimState> for SimRenderState {
//     fn from(value: SimState) -> Self {
//         match value {
//             SimState::Closed => Self::Closed,
//             SimState::Init => Self::Init,
//             SimState::Paused => Self::Paused,
//             SimState::Running => Self::Running,
//         }
//     }
// }

// Intialized through the UI.
#[derive(Resource, Clone, Debug, PartialEq, Derivative)]
#[derivative(Default)]
pub struct SimSettings {
    pub teams: Vec<Team>,
    pub players: Vec<Player>,
    pub parent_node: Option<Entity>,
    #[derivative(Default(value = "32"))]
    pub size: u32, // Must be a power of 2.
    #[derivative(Default(value = "10"))]
    pub timestep: u32, // fps
    #[derivative(Default(value = "100"))]
    pub steps_per_turn: u32,
    pub layout: SimLayout,
}

#[derive(Default, Debug, strum::Display, Copy, Clone, PartialEq)]
pub enum SimLayout {
    #[default]
    Random,
    #[strum(to_string = "50/50 Horizontal")]
    Horiz5050,
    #[strum(to_string = "50/50 Vertical")]
    Vert5050,
    #[strum(to_string = "50/50 Random")]
    Rand5050,
    Empty,
}
impl TryFrom<&String> for SimLayout {
    type Error = anyhow::Error;
    fn try_from(value: &String) -> anyhow::Result<Self> {
        match value.to_string().as_str() {
            "Random" => Ok(Self::Random),
            "50/50 Horizontal" => Ok(Self::Horiz5050),
            "50/50 Vertical" => Ok(Self::Vert5050),
            "50/50 Random" => Ok(Self::Rand5050),
            "Empty" => Ok(Self::Empty),
            _ => Err(anyhow::anyhow!("No such layout")),
        }
    }
}

#[derive(Resource, Clone, Default, Debug, PartialEq, Deref, DerefMut, ExtractResource)]
pub struct UseCompute(pub bool);

pub struct SimPlugin;
impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.add_plugins(web::SoftwareSimPlugin)
                .insert_resource(SimSettings {
                    teams: vec![
                        Team {
                            id: 0,
                            name: "A".into(),
                            players: Vec::new(),
                            color: [255, 0, 0, 255],
                        },
                        Team {
                            id: 1,
                            name: "B".into(),
                            players: Vec::new(),
                            color: [0, 0, 255, 255],
                        },
                    ],
                    ..Default::default()
                })
                .init_resource::<SimImages>()
                .init_resource::<UseCompute>()
                .init_resource::<SimGameplayState>()
                // .init_resource::<SimRenderState>()
                // .add_plugins(ShaderSimPlugin)
                .insert_resource(Time::<Fixed>::from_hz(10.))
                .init_state::<SimState>()
                // .add_systems(StateTransition, set_sim_render_state)
                .add_systems(
                    OnEnter(SimState::Init),
                    (init_images, spawn_sprite, init_timestep).chain(),
                )
                .add_systems(OnEnter(SimState::Running), unpause)
                .add_systems(OnEnter(SimState::Paused), pause)
                .add_systems(OnEnter(SimState::Closed), cleanup)
                .configure_sets(
                    Update,
                    (
                        SoftwareSimSet.run_if(resource_exists_and_equals(UseCompute(false))),
                        // ShaderSimSet.run_if(resource_exists_and_equals(UseCompute(true))),
                    ),
                )
                .configure_sets(
                    OnEnter(SimState::Init),
                    (
                        SoftwareSimSet.run_if(resource_exists_and_equals(UseCompute(false))),
                        // ShaderSimSet.run_if(resource_exists_and_equals(UseCompute(true))),
                    ),
                )
                .add_observer(on_stamp)
        };
        // app.sub_app_mut(RenderApp).configure_sets(
        //     Render,
        //     ShaderSimSet.run_if(resource_exists_and_equals(UseCompute(true))),
        // );
    }
}

/// Double buffer.
#[derive(Debug, Resource, Clone, ExtractResource, Default)]
pub struct SimImages {
    pub texture_a: Handle<Image>,
    pub texture_b: Handle<Image>,
    pub preview_texture: Handle<Image>,
}
fn init_images(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    // use_compute: Res<UseCompute>,
    settings: Res<SimSettings>,
) {
    let (asset_usage, format) = if cfg!(feature = "compute_shaders") {
        // !NB! compute shader should reflect this
        (RenderAssetUsages::RENDER_WORLD, TextureFormat::Rgba8Unorm)
    } else {
        (
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
            TextureFormat::Rgba8UnormSrgb,
        )
    };
    let mut image = Image::new_fill(
        Extent3d {
            width: settings.size,
            height: settings.size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        format,
        asset_usage,
    );
    image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING;
    #[cfg(feature = "compute_shaders")]
    {
        image.texture_descriptor.usage |= TextureUsages::STORAGE_BINDING;
    }

    commands.insert_resource(SimImages {
        preview_texture: images.add(image.clone()),
        texture_a: images.add(image.clone()),
        texture_b: images.add(image),
    });
}

fn init_timestep(mut time: ResMut<Time<Fixed>>, settings: Res<SimSettings>) {
    time.set_timestep_hz(settings.timestep as f64);
}

fn unpause(
    mut time: ResMut<Time<Virtual>>,
    mut image_node: Single<&mut ImageNode, With<SimImageNode>>,
    sim_images: Res<SimImages>,
    mut images: ResMut<Assets<Image>>,
) {
    time.unpause();
    image_node.image = sim_images.texture_a.clone();
    let preview_image = images
        .get_mut(&sim_images.preview_texture)
        .expect("preview texture")
        .clone();
    images
        .get_mut(&sim_images.texture_a)
        .expect("tex_a")
        .clone_from(&preview_image);
    images
        .get_mut(&sim_images.texture_b)
        .expect("tex_b")
        .clone_from(&preview_image);
}

fn pause(
    mut time: ResMut<Time<Virtual>>,
    sim_imgs: ResMut<SimImages>,
    settings: Res<SimSettings>,
    mut image_nodes: Query<&mut ImageNode>,
    mut images: ResMut<Assets<Image>>,
) {
    time.pause();
    let mut parent_node = settings
        .parent_node
        .and_then(|p| image_nodes.get_mut(p).ok())
        .expect("parent handle");
    let current_handle = parent_node.image.clone();
    let current_img = images.get(&current_handle).expect("current_img").clone();
    images
        .get_mut(&sim_imgs.texture_a)
        .expect("tex_a")
        .clone_from(&current_img);
    images
        .get_mut(&sim_imgs.texture_b)
        .expect("tex_b")
        .clone_from(&current_img);
    images
        .get_mut(&sim_imgs.preview_texture)
        .expect("tex_p")
        .clone_from(&current_img);

    parent_node.image = sim_imgs.preview_texture.clone();
}

#[derive(Component)]
pub struct SimSprite;

pub fn spawn_sprite(
    mut commands: Commands,
    images: Res<SimImages>,
    settings: Res<SimSettings>,
    mut image_nodes: Query<&mut ImageNode>,
) {
    if let Some(parent_node) = settings.parent_node {
        let mut parent = image_nodes
            .get_mut(parent_node)
            .expect("Could not get parent node!");
        parent.image = images.texture_a.clone();
        commands
            .get_entity(parent_node)
            .expect("get_entity")
            .insert(SimSprite);
    }
}
pub fn cleanup(mut commands: Commands, query: Single<Entity, With<SimSprite>>) {
    commands.get_entity(query.entity()).unwrap().despawn();
}

// fn set_sim_render_state(state: Res<State<SimState>>, mut render_state: ResMut<SimRenderState>) {
//     *render_state = (*state.get()).into()
// }

// fn on_stamp(
//     trigger: Trigger<StampEvent>,
//     gameplay_state: Res<SimGameplayState>,
//     settings: Res<SimSettings>,
//     image_nodes: Query<&ImageNode>,
//     mut images: ResMut<Assets<Image>>,
//     stamps: Res<Stamps>,
//     stamp_assets: Res<Assets<Stamp>>,
//     atlases: Res<Assets<TextureAtlasLayout>>,
//     mut sim_state: ResMut<NextState<SimState>>,
// ) {
//     let res = (|| {
//         let stamps = stamps.get_from_sim_size(settings.size);
//         let stamp_size = Stamps::stamp_size_from_sim_size(settings.size) as f32;
//         let stamp_name = gameplay_state.current_stamp.as_ref()?;
//         let stamp = stamps.get(stamp_name)?;
//         let stamp = stamp_assets.get(stamp)?;
//         let stamp_data = stamp.get_pixel_data(&images, &atlases);
//         info!(
//             "size: {}\nstamp_size: {stamp_size}\nstamp_name: {stamp_name}\nstamp: {stamp:#?}",
//             settings.size
//         );

//         let parent_node = settings.parent_node?;
//         let node = image_nodes.get(parent_node).ok()?;
//         let sim_img = images.get_mut(&node.image)?;
//         let pos = trigger.position * Vec2::splat(settings.size as f32);

//         let mid = stamp_size / 2.;
//         let range = |pos| (pos - mid) as u32..(pos + mid) as u32;
//         for (stamp_x, sim_x) in range(pos.x).enumerate() {
//             for (stamp_y, sim_y) in range(pos.y).enumerate() {
//                 let stamp_color = &stamp_data[stamp_x][stamp_y];
//                 if stamp_color[3] != 0 {
//                     sim_img
//                         .set_color_at(
//                             sim_x,
//                             sim_y,
//                             Color::srgb_u8(stamp_color[0], stamp_color[1], stamp_color[2]),
//                         )
//                         .expect("set_color");
//                 }
//             }
//         }

//         sim_state.set(SimState::Running);

//         Some(())
//     })();
//     if res.is_none() {
//         warn!("Could not apply stamp.");
//     };
// }

fn on_stamp(_trigger: Trigger<StampEvent>, mut state: ResMut<NextState<SimState>>) {
    state.set(SimState::Running);
}
