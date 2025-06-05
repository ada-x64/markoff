//! The Native simulation uses compute shaders.

use bevy::{
    prelude::*,
    render::{
        Render, RenderApp, RenderSet, extract_resource::ExtractResourcePlugin,
        graph::CameraDriverLabel, render_graph::RenderGraph,
    },
};

pub mod render;
pub use render::*;

pub mod shader;
pub use shader::*;

use crate::sim::SimImages;

pub const DISPLAY_FACTOR: u32 = 1;
pub const IMG_SIZE: u32 = 512;
pub const SIM_SIZE: u32 = IMG_SIZE / DISPLAY_FACTOR;
pub const WORKGROUP_SIZE: u32 = 8; // workgroup = num threads
pub const SHADER_ASSET_PATH: &str = "shader/simulation.wgsl";

pub struct InnerSimPlugin;
impl Plugin for InnerSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractResourcePlugin::<SimImages>::default(),
            ExtractResourcePlugin::<SimState>::default(),
            ExtractResourcePlugin::<UseCompute>::default(),
        ))
        .add_systems(FixedUpdate, swap_buffer.in_set(ShaderSimSet));
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            prepare_bind_group.in_set(RenderSet::PrepareBindGroups),
        );

        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(SimLabel, SimulationNode::default());
        render_graph.add_node_edge(SimLabel, CameraDriverLabel)
    }
    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<SimPipeline>();
    }
}
