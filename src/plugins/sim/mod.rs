use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
};
use derivative::Derivative;

use crate::{
    sim::{
        // native::{ShaderSimPlugin, ShaderSimSet},
        web::{SoftwareSimPlugin, SoftwareSimSet},
    },
    teams::types::{Player, Team},
};

// TODO: This is crashing!
// But we have bigger fish to fry right now.
// mod native;
mod web;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SimState {
    #[default]
    Closed,
    Init,
    Paused,
    Running,
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
    pub speed: u32, // fps
}

#[derive(Resource, Clone, Default, Debug, PartialEq, Deref, DerefMut, ExtractResource)]
pub struct UseCompute(pub bool);

pub struct SimPlugin;
impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.add_plugins(SoftwareSimPlugin)
                .init_resource::<SimSettings>()
                .init_resource::<UseCompute>()
                // .init_resource::<SimRenderState>()
                // .add_plugins(ShaderSimPlugin)
                .insert_resource(Time::<Fixed>::from_hz(10.))
                .init_state::<SimState>()
                // .add_systems(StateTransition, set_sim_render_state)
                .add_systems(OnEnter(SimState::Init), (init_images, spawn_sprite).chain())
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
        };
        // app.sub_app_mut(RenderApp).configure_sets(
        //     Render,
        //     ShaderSimSet.run_if(resource_exists_and_equals(UseCompute(true))),
        // );
    }
}

/// Double buffer.
#[derive(Debug, Resource, Clone, ExtractResource)]
pub struct SimImages {
    pub texture_a: Handle<Image>,
    pub texture_b: Handle<Image>,
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
        &[0, 0, 0, 255],
        format,
        asset_usage,
    );
    image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING;
    #[cfg(feature = "compute_shaders")]
    {
        image.texture_descriptor.usage |= TextureUsages::STORAGE_BINDING;
    }

    commands.insert_resource(SimImages {
        texture_a: images.add(image.clone()),
        texture_b: images.add(image),
    });
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
