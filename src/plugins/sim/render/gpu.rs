//! The Native simulation uses compute shaders.

use crate::sim::{data::*, run_gpu_systems};
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

pub struct GpuSimPlugin;
impl Plugin for GpuSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ExtractResourcePlugin::<SimImages>::default(),))
            .add_plugins((ExtractResourcePlugin::<SimSettings>::default(),))
            .add_systems(FixedUpdate, swap_buffer.in_set(GpuSimSystems));
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(
                Render,
                prepare_bind_group
                    .in_set(RenderSet::PrepareBindGroups)
                    .run_if(run_gpu_systems),
            )
            // .add_systems(PreUpdate, route_state)
            .add_systems(OnEnter(SimState::Init), init.run_if(run_gpu_systems))
            .add_systems(OnEnter(SimState::Closed), cleanup.run_if(run_gpu_systems));
    }
    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<SimPipeline>();
    }
}

fn init(mut render_graph: ResMut<RenderGraph>) {
    render_graph.add_node(SimLabel, SimulationNode::default());
    render_graph.add_node_edge(SimLabel, CameraDriverLabel);
}
fn cleanup(mut render_graph: ResMut<RenderGraph>) {
    render_graph.remove_node(SimLabel).unwrap();
    render_graph
        .remove_node_edge(SimLabel, CameraDriverLabel)
        .unwrap();
}

fn swap_buffer(mut sprite: Single<&mut ImageNode, With<SimSprite>>, imgs: Res<SimImages>) {
    if sprite.image == imgs.texture_a {
        sprite.image = imgs.texture_b.clone();
    } else {
        sprite.image = imgs.texture_a.clone();
    }
}
