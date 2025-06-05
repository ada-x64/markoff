use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
};

#[cfg(not(feature = "compute_shaders"))]
mod web;
#[cfg(not(feature = "compute_shaders"))]
use web::*;

#[cfg(feature = "compute_shaders")]
mod native;
#[cfg(feature = "compute_shaders")]
use native::*;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SimState {
    #[default]
    Closed,
    Init,
    Paused,
    Running,
}

#[derive(Resource, Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct SimSettings {
    use_compute: bool,
}

pub struct SimPlugin;
impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.insert_resource(SimSettings {
                use_compute: !cfg!(target_arch = "wasm32"),
            })
            .insert_resource(Time::<Fixed>::from_hz(5.))
            .init_resource::<SimImages>()
            .init_state::<SimState>()
            .add_plugins(InnerSimPlugin)
            .add_systems(OnEnter(SimState::Init), spawn_sprite)
            .add_systems(OnEnter(SimState::Closed), cleanup)
        };
    }
}

/// Double buffer.
#[derive(Debug, Resource, Clone, ExtractResource)]
pub struct SimImages {
    pub texture_a: Handle<Image>,
    pub texture_b: Handle<Image>,
}
impl FromWorld for SimImages {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.get_resource_mut::<Assets<Image>>().unwrap();
        let mut image = Image::new_fill(
            Extent3d {
                width: SIM_SIZE,
                height: SIM_SIZE,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[0, 0, 0, 255],
            // !NB! compute shader should reflect this
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );
        image.texture_descriptor.usage = TextureUsages::COPY_DST
            // | TextureUsages::STORAGE_BINDING
            | TextureUsages::TEXTURE_BINDING;
        let texture_a = images.add(image.clone());
        let texture_b = images.add(image);
        Self {
            texture_a,
            texture_b,
        }
    }
}

#[derive(Component)]
pub struct SimSprite;

pub fn spawn_sprite(mut commands: Commands, images: Res<SimImages>, assets: Res<Assets<Image>>) {
    let handle = &images.texture_a;
    let img = assets.get(handle.id()).unwrap();
    commands.spawn((
        Node {
            width: Val::Px(img.width() as f32),
            height: Val::Px(img.height() as f32),
            ..Default::default()
        },
        Outline::new(Val::Px(2.), Val::Px(2.), Color::srgb(1., 0., 0.)),
        children![(SimSprite, ImageNode::new(handle.clone()),)],
    ));
}
pub fn cleanup(mut commands: Commands, query: Single<Entity, With<SimSprite>>) {
    commands.get_entity(query.entity()).unwrap().despawn();
}
