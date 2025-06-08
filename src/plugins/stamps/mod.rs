use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Clone, Debug)]
pub struct Stamp {
    pub atlas: TextureAtlas,
    pub texture: Handle<Image>,
}

#[derive(Resource, Clone, Debug, Default)]
pub struct Stamps {
    pub px8: HashMap<String, Stamp>,
    pub px16: HashMap<String, Stamp>,
    pub px32: HashMap<String, Stamp>,
}
impl Stamps {
    pub fn get_from_size(&self, size: u32) -> &HashMap<String, Stamp> {
        if size > 128 {
            &self.px32
        } else if size > 64 {
            &self.px16
        } else {
            &self.px8
        }
    }
    pub fn get_from_size_mut(&mut self, size: u32) -> &mut HashMap<String, Stamp> {
        if size > 128 {
            &mut self.px32
        } else if size > 64 {
            &mut self.px16
        } else {
            &mut self.px8
        }
    }
}

pub struct StampPlugin;
impl Plugin for StampPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init).init_resource::<Stamps>();
    }
}

fn init(
    assets: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut stamps: ResMut<Stamps>,
) {
    for size in [8, 16, 32] {
        let texture = assets.load(format!("sprites/stamps/{size}px.png"));
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(size), 5, 1, None, None);
        let layout = layouts.add(layout.clone());
        let stamps = stamps.get_from_size_mut(size);
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
            };
            stamps.insert(format!("{name} ({size}px)"), stamp);
        }
    }
}
