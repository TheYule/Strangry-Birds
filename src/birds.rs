use avian2d::prelude::{Collider, RigidBody};
use bevy::{
    asset::AssetServer, ecs::bundle::Bundle, math::Vec2, prelude::Component, sprite::Sprite,
    utils::default,
};

use crate::common::{Despawn, SCALE};

#[derive(Clone, Component)]
#[require(Despawn)]
pub enum Bird {
    Strang,
    Michael,
    BloatedStrang,
}

impl Bird {
    pub fn asset_path(&self) -> &str {
        match self {
            Bird::Strang => "birds/strang.png",
            Bird::Michael => "birds/michael.png",
            Bird::BloatedStrang => "birds/bloated_strang.png",
        }
    }

    pub fn size(&self) -> f32 {
        match self {
            Bird::Strang => SCALE / 2.0,
            Bird::Michael => SCALE / 2.0,
            Bird::BloatedStrang => SCALE,
        }
    }

    pub fn spawn_ghost(&self, asset_server: &AssetServer) -> impl Bundle {
        (
            self.clone(),
            Sprite {
                image: asset_server.load(self.asset_path().to_string()),
                custom_size: Some(Vec2::splat(self.size())),
                ..default()
            },
        )
    }
    
    pub fn spawn(&self, asset_server: &AssetServer) -> impl Bundle {
        (
            self.spawn_ghost(asset_server),
            RigidBody::Dynamic,
            Collider::circle(self.size())
        )
    }
}

#[derive(Component)]
pub struct CurrentBird;
