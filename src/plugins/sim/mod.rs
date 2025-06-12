use bevy::prelude::*;

pub use data::*;

use crate::sim::{lifecycle::SimLifecyclePlugin, render::cpu::CpuSimPlugin};

mod data;
mod lifecycle;
mod render;

pub struct SimPlugin;
impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                app.add_plugins(crate::sim::render::gpu::GpuSimPlugin);
            }
            app.add_plugins((CpuSimPlugin, SimLifecyclePlugin))
                .insert_resource(SimSettings {
                    teams: vec![
                        Team {
                            id: 0,
                            name: "A".into(),
                            players: vec![0],
                            color: [255, 0, 0, 255],
                        },
                        Team {
                            id: 1,
                            name: "B".into(),
                            players: vec![1],
                            color: [0, 0, 255, 255],
                        },
                    ],
                    players: vec![
                        Player {
                            name: "Player 1".into(),
                            team: 0,
                        },
                        Player {
                            name: "Player 2".into(),
                            team: 1,
                        },
                    ],
                    ..Default::default()
                })
                .init_resource::<SimImages>()
                .init_resource::<SimGameplayState>()
                // todo: crashing
                // this is super annoying!
                .configure_sets(
                    Update,
                    (
                        GpuSimSystems.run_if(run_gpu_systems),
                        CpuSimSystems.run_if(run_cpu_systems),
                    ),
                )
                .configure_sets(
                    FixedUpdate,
                    (
                        GpuSimSystems.run_if(run_gpu_systems),
                        CpuSimSystems.run_if(run_cpu_systems),
                    ),
                )
                .configure_sets(
                    OnEnter(SimState::Init),
                    (
                        GpuSimSystems.run_if(run_gpu_systems),
                        CpuSimSystems.run_if(run_cpu_systems),
                    ),
                )
                .configure_sets(
                    OnEnter(SimState::Running),
                    (
                        GpuSimSystems.run_if(run_gpu_systems),
                        CpuSimSystems.run_if(run_cpu_systems),
                    ),
                )
                .configure_sets(
                    OnEnter(SimState::Paused),
                    (
                        GpuSimSystems.run_if(run_gpu_systems),
                        CpuSimSystems.run_if(run_cpu_systems),
                    ),
                )
                .configure_sets(
                    OnEnter(SimState::Closed),
                    (
                        GpuSimSystems.run_if(run_gpu_systems),
                        CpuSimSystems.run_if(run_cpu_systems),
                    ),
                )
        };
    }
}

fn run_gpu_systems(settings: Res<SimSettings>) -> bool {
    settings.use_compute
}
fn run_cpu_systems(settings: Res<SimSettings>) -> bool {
    !settings.use_compute
}
