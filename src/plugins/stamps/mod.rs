use anyhow::anyhow;
use bevy::{image::TextureAccessError, platform::collections::HashMap, prelude::*};
use itertools::Itertools;

#[derive(Clone, Debug, Asset, Reflect)]
pub struct Stamp {
    pub atlas: TextureAtlas,
    pub texture: Handle<Image>,
    pub name: String,
    pub size: u32,
}
impl Stamp {
    pub fn get_pixel_data(
        &self,
        images: &Assets<Image>,
        atlases: &Assets<TextureAtlasLayout>,
    ) -> anyhow::Result<Vec<Vec<Vec<u8>>>> {
        let tex = images.get(&self.texture).ok_or(anyhow!("texture"))?;
        let rect = self.atlas.texture_rect(atlases).ok_or(anyhow!("atlas"))?;
        let res = (rect.min.x..rect.max.x)
            .map(|x| {
                (rect.min.y..rect.max.y)
                    .map(|y| {
                        tex.pixel_bytes(UVec3::new(x, y, 0))
                            .expect("out of range!")
                            .to_owned()
                    })
                    .collect_vec()
            })
            .collect_vec();
        Ok(res)
    }
    pub fn add_to_texture<'a>(
        &'a self,
        texture: &'a mut Image,
        pos: Vec2,
        images: &Assets<Image>,
        atlases: &Assets<TextureAtlasLayout>,
    ) -> anyhow::Result<&'a mut Image> {
        let mid = self.size as f32 / 2.;
        let range = |pos| (pos - mid) as u32..(pos + mid) as u32;
        let data = &self.get_pixel_data(images, atlases)?;
        for (stamp_x, sim_x) in range(pos.x).enumerate() {
            for (stamp_y, sim_y) in range(pos.y).enumerate() {
                let color = &data[stamp_x][stamp_y];
                if color[3] != 0 {
                    match texture.set_color_at(
                        sim_x,
                        sim_y,
                        Color::srgb_u8(color[0], color[1], color[2]),
                    ) {
                        Err(TextureAccessError::OutOfBounds { x: _, y: _, z: _ }) | Ok(_) => {}
                        Err(e) => {
                            error!("{e:#?}");
                        }
                    }
                }
            }
        }
        Ok(texture)
    }
}

#[derive(Resource, Clone, Debug, Default)]
pub struct Stamps {
    pub px8: HashMap<String, Handle<Stamp>>,
    pub px16: HashMap<String, Handle<Stamp>>,
    pub px32: HashMap<String, Handle<Stamp>>,
}
#[allow(unused)]
impl Stamps {
    /// Square.
    pub fn stamp_size_from_sim_size(size: u32) -> u32 {
        match size {
            0..=32 => 8,
            33..=64 => 16,
            _ => 32,
        }
    }
    pub fn get_from_stamp_size(&self, size: u32) -> &HashMap<String, Handle<Stamp>> {
        if size == 32 {
            &self.px32
        } else if size == 16 {
            &self.px16
        } else {
            &self.px8
        }
    }
    pub fn get_from_stamp_size_mut(&mut self, size: u32) -> &mut HashMap<String, Handle<Stamp>> {
        if size == 32 {
            &mut self.px32
        } else if size == 16 {
            &mut self.px16
        } else {
            &mut self.px8
        }
    }
    pub fn get_from_sim_size(&self, size: u32) -> &HashMap<String, Handle<Stamp>> {
        let size = Self::stamp_size_from_sim_size(size);
        self.get_from_stamp_size(size)
    }
    pub fn get_from_sim_size_mut(&mut self, size: u32) -> &mut HashMap<String, Handle<Stamp>> {
        let size = Self::stamp_size_from_sim_size(size);
        self.get_from_stamp_size_mut(size)
    }
}

pub struct StampPlugin;
impl Plugin for StampPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .init_asset::<Stamp>()
            .init_resource::<Stamps>();
    }
}

fn init(
    assets: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut stamps: ResMut<Stamps>,
    mut stamp_assets: ResMut<Assets<Stamp>>,
) {
    for size in [8, 16, 32] {
        let texture = assets.load(format!("sprites/stamps/{size}px.png"));
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(size), 5, 1, None, None);
        let layout = layouts.add(layout.clone());
        let stamps = stamps.get_from_stamp_size_mut(size);
        for (i, name) in ["Square", "Noise", "Star", "Diag 1", "Diag 2"]
            .iter()
            .enumerate()
        {
            let stamp = Stamp {
                texture: texture.clone(),
                atlas: TextureAtlas {
                    layout: layout.clone(),
                    index: i,
                },
                name: format!("{name} ({size}px)"),
                size,
            };
            let handle = stamp_assets.add(stamp);
            stamps.insert(name.to_string(), handle);
        }
    }
}
