//! The web module uses CPU based rendering.

use std::sync::{Arc, Mutex, RwLock};

use bevy::{prelude::*, tasks::ComputeTaskPool};

use crate::{
    cells::types::{CellCondition, CellResult},
    sim::{SimImages, SimSprite, SimState, UseCompute, spawn_sprite},
};

pub const DISPLAY_FACTOR: u32 = 8;
pub const IMG_SIZE: u32 = 512;
pub const SIM_SIZE: u32 = IMG_SIZE / DISPLAY_FACTOR;
pub const NUM_WORKGROUPS: usize = 8;
pub const CHUNK_SIZE: usize = (SIM_SIZE / NUM_WORKGROUPS as u32) as usize;

type PixelColor<'a> = &'a [u8; 4];
const BLACK: PixelColor = &[0, 0, 0, 255];
const WHITE: PixelColor = &[255, 255, 255, 255];

#[derive(SystemSet, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SoftwareSimSet;

pub struct SoftwareSimPlugin;
impl Plugin for SoftwareSimPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.add_systems(
                FixedUpdate,
                (draw)
                    .run_if(in_state(SimState::Running))
                    .in_set(SoftwareSimSet),
            )
            .add_systems(
                OnEnter(SimState::Init),
                init.after(spawn_sprite).in_set(SoftwareSimSet),
            )
        };
    }
}

// populate with noise
fn init(
    sprite: Single<&ImageNode, With<SimSprite>>,
    mut images: ResMut<Assets<Image>>,
    mut next: ResMut<NextState<SimState>>,
) {
    let img = images.get_mut(sprite.image.id()).unwrap();
    let size = img.size();
    for x in 0..size.x {
        for y in 0..size.y {
            let color = if rand::random_bool(0.1) { WHITE } else { BLACK };
            img.set_color_at(x, y, Color::srgb_u8(color[0], color[1], color[2]))
                .unwrap();
        }
    }
    next.set(SimState::Running);
}

fn draw(
    mut sprite: Single<&mut ImageNode, With<SimSprite>>,
    image_handles: Res<SimImages>,
    mut images: ResMut<Assets<Image>>,
) {
    // This calculation function should be passed in.
    // It should be the team's chosen rule.
    // Some restrictions: The neighborhood _must_ have at least one active cell.
    let calculate = |current: CellCondition, neighborhood: [CellCondition; 8]| -> CellResult {
        let num_active = neighborhood
            .iter()
            .filter(|c| **c == CellCondition::Active)
            .count();
        let spawn = current == CellCondition::Empty && num_active == 3;
        let stay_alive = num_active == 2 || num_active == 3;
        let die = current == CellCondition::Active && !(2..=3).contains(&num_active);

        if spawn || stay_alive {
            CellResult::Active
        } else if die {
            CellResult::Empty
        } else {
            CellResult::Untouched
        }
    };
    // this should also be passed in for the current team
    // stored as a resource or an entity
    let team_color: PixelColor = &[255, 0, 0, 255];

    let next_handle = if sprite.image == image_handles.texture_a {
        image_handles.texture_b.clone()
    } else {
        image_handles.texture_a.clone()
    };
    let next_img = images.remove(next_handle.id()).expect("next_img");
    let write_lock = Arc::new(Mutex::new(next_img));

    let current_img = images.get(sprite.image.id()).expect("current_img");
    let size = current_img.size();
    let read_lock = Arc::new(RwLock::new(current_img));

    // NOTE: This is spawning and closing threads every frame.
    // Would be better to use a consistent threadpool and pass messages.
    let num_chunks = (size.x * size.y) / CHUNK_SIZE as u32;
    let pool = ComputeTaskPool::get();
    pool.scope(|scope| {
        for chunk_idx in 0..num_chunks {
            let read_lock = read_lock.clone();
            let write_lock = write_lock.clone();
            scope.spawn(async move {
                let read_lock = read_lock.read().unwrap();
                let size = read_lock.size();
                let mut iterator = (0..size.x * size.y).map(|i| {
                    let bytes = read_lock
                        .pixel_bytes(UVec3::new(i % size.x, i / size.y, 0))
                        .expect("pixel_bytes")
                        .to_owned();
                    let bytes = bytes
                        .leak()
                        .as_array::<4>()
                        .expect("cast pixel_bytes to array[4]");
                    (i, bytes)
                });
                iterator
                    .advance_by(chunk_idx as usize * CHUNK_SIZE)
                    .expect("advance_by");
                let chunk = iterator.next_chunk::<CHUNK_SIZE>().expect("next_chunk");
                let colors = chunk.map(|(i, cell)| {
                    let i = i as i32;
                    let w = size.x as i32;
                    #[rustfmt::skip]
                    let neighborhood = [
                        i - 1 - w,  i - w,  i + 1 - w,
                        i - 1,              i + 1,
                        i - 1 + w,  i + w,  i + 1 + w
                    ]
                    .map(|i| {
                        if i < 0 {
                            BLACK
                        } else {
                            let bytes = read_lock.pixel_bytes(UVec3::new(i as u32 % SIM_SIZE, i as u32 / SIM_SIZE, 0))
                                .unwrap_or(BLACK);
                            bytes.as_array::<4>().expect("cast to array")
                        }
                    })
                    .map(|cell| get_condition(cell, team_color));
                    let res = calculate(get_condition(cell, team_color), neighborhood);
                    let color = match res {
                        CellResult::Empty => BLACK,
                        CellResult::Active => WHITE,
                        CellResult::Untouched => cell,
                    };
                    (i, color)
                });
                drop(read_lock);
                let mut write_lock = write_lock.lock().unwrap();
                let data = write_lock.data.as_mut().expect("data");
                colors.into_iter().for_each(|(i, color)| {
                    let offset = i as usize * 4;
                    data[offset] = color[0];
                    data[offset + 1] = color[1];
                    data[offset + 2] = color[2];
                    data[offset + 3] = color[3];
                });
            });
        }
    });
    let val = Arc::try_unwrap(write_lock).unwrap().into_inner().unwrap();
    images.insert(next_handle.id(), val);
    sprite.image = next_handle;
}

fn get_condition(pixel: PixelColor, team_color: PixelColor) -> CellCondition {
    if pixel == team_color {
        CellCondition::Owned
    } else if pixel == BLACK {
        CellCondition::Empty
    } else if pixel == WHITE {
        CellCondition::Active
    } else {
        CellCondition::Enemy
    }
}
