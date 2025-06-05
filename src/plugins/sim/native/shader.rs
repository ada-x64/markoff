//! Set up the pipelines and bind groups, set SHADER_ASSET_PATH.
//! Largely inspired by the Game Of Life example.
//! https://github.com/bevyengine/bevy/blob/main/examples/shader/compute_shader_game_of_life.rs

use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries,
            CachedComputePipelineId, ComputePipelineDescriptor, PipelineCache, ShaderStages,
            StorageTextureAccess, TextureFormat, binding_types::texture_storage_2d,
        },
        renderer::RenderDevice,
        texture::GpuImage,
    },
};

use crate::sim::native::{SHADER_ASSET_PATH, SimImages};

#[derive(Resource)]
pub struct SimPipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub init_pipeline: CachedComputePipelineId,
    pub update_pipeline: CachedComputePipelineId,
}

impl FromWorld for SimPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = render_device.create_bind_group_layout(
            "SimulationImages",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_2d(
                        TextureFormat::Rgba8UnormSrgb,
                        StorageTextureAccess::ReadOnly,
                    ),
                    texture_storage_2d(
                        TextureFormat::Rgba8UnormSrgb,
                        StorageTextureAccess::WriteOnly,
                    ),
                ),
            ),
        );
        let shader = world.load_asset(SHADER_ASSET_PATH);
        let pipeline_cache = world.resource::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
            zero_initialize_workgroup_memory: true,
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
            zero_initialize_workgroup_memory: true,
        });

        SimPipeline {
            init_pipeline,
            texture_bind_group_layout,
            update_pipeline,
        }
    }
}

#[derive(Resource)]
pub struct SimBindGroups(pub [BindGroup; 2]);

pub fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<SimPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    images: Res<SimImages>,
    render_device: Res<RenderDevice>,
) {
    let view_a = gpu_images.get(&images.texture_a).unwrap();
    let view_b = gpu_images.get(&images.texture_b).unwrap();
    let bind_group_0 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((&view_a.texture_view, &view_b.texture_view)),
    );
    let bind_group_1 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((&view_b.texture_view, &view_a.texture_view)),
    );
    commands.insert_resource(SimBindGroups([bind_group_0, bind_group_1]));
}
