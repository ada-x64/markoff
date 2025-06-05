//! The web module uses CPU based rendering.

use std::sync::{Arc, Mutex, RwLock};

use bevy::{prelude::*, tasks::ComputeTaskPool};

use crate::{
    cells::types::{CellCondition, CellResult},
    sim::{SimImages, SimSprite, SimState, spawn_sprite},
};

pub const DISPLAY_FACTOR: u32 = 1;
pub const IMG_SIZE: u32 = 512;
pub const SIM_SIZE: u32 = IMG_SIZE / DISPLAY_FACTOR; // 512
pub const NUM_WORKGROUPS: usize = 8;
pub const CHUNK_SIZE: usize = (SIM_SIZE / NUM_WORKGROUPS as u32) as usize;

type PixelColor<'a> = &'a [u8];
const BLACK: PixelColor = &[0, 0, 0];
const WHITE: PixelColor = &[255, 255, 255];

pub struct InnerSimPlugin;
impl Plugin for InnerSimPlugin {
    fn build(&self, app: &mut App) {
        let _ = {
            app.add_systems(FixedUpdate, (draw).run_if(in_state(SimState::Running)))
                .add_systems(OnEnter(SimState::Init), init.after(spawn_sprite))
        };
    }
}

// populate with noise
fn init(
    sprite: Single<&ImageNode, With<SimSprite>>,
    mut images: ResMut<Assets<Image>>,
    mut next: ResMut<NextState<SimState>>,
) {
    info_once!("Init!");
    let img = images.get_mut(sprite.image.id()).unwrap();
    let size = img.size();
    for x in 0..size.x {
        for y in 0..size.y {
            let color = if rand::random_bool(0.1) { WHITE } else { BLACK };
            img.set_color_at(x, y, Color::srgb_u8(color[0], color[1], color[2]))
                .unwrap();
        }
    }
    info_once!("Initialized image! {img:?}");
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
    let team_color: PixelColor = &[255, 0, 0];

    let next_handle = if sprite.image == image_handles.texture_a {
        image_handles.texture_b.clone()
    } else {
        image_handles.texture_a.clone()
    };
    let next_img = images.remove(next_handle.id()).expect("next_img");
    let write_lock = Arc::new(Mutex::new(next_img));

    let current_img = images.get(sprite.image.id()).expect("current_img");
    let read_lock = Arc::new(RwLock::new(current_img));

    let pool = ComputeTaskPool::get();
    pool.scope(|scope| {
        for chunk_idx in 0..NUM_WORKGROUPS {
            let read_lock = read_lock.clone();
            let write_lock = write_lock.clone();
            scope.spawn(async move {
                let img = read_lock.read().unwrap();
                let size = img.size();
                let mut iterator = (0..size.x * size.y).map(|i| {
                    (
                        i,
                        img.pixel_bytes(UVec3::new(i % size.x, i / size.y, 0)).expect("pixel_bytes"),
                    )
                });
                iterator.advance_by(chunk_idx * CHUNK_SIZE).expect("advance_by");
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
                            img.pixel_bytes(UVec3::new(i as u32 % SIM_SIZE, i as u32 / SIM_SIZE, 0)).unwrap_or(BLACK)
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
                let mut write_lock = write_lock.lock().unwrap();
                colors.iter().for_each(|(i, color)| {
                    write_lock
                        .set_color_at(
                            *i as u32 % SIM_SIZE,
                            *i as u32 / SIM_SIZE,
                            Color::srgb_u8(color[0], color[1], color[2]),
                        )
                        .expect("write");
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
