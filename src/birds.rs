use std::ops::Sub;

use avian2d::prelude::{Collider, LinearDamping, RigidBody};
use bevy::{
    asset::AssetServer,
    ecs::{bundle::Bundle, entity::Entity, system::Commands},
    math::{Vec2, Vec3},
    prelude::Component,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

use crate::{
    common::{Despawn, SCALE},
    slingshot::Slingshot,
};

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
            Collider::circle(self.size() / 2.0),
            LinearDamping(0.2),
        )
    }
}

#[derive(Component)]
pub struct CurrentBird;

#[derive(Component)]
pub struct BirdOrder(pub usize);

pub fn spawn_birds(
    birds: &Vec<Bird>,
    slingshot_transform: &Transform,
    mut commands: Commands,
    asset_server: &AssetServer,
) -> Vec<Entity> {
    let mut entities = vec![];
    let mut i = 0;

    for bird in birds {
        let mut b =
            commands.spawn((
                bird.spawn_ghost(asset_server),
                BirdOrder(i),
                Transform::from_translation(Slingshot::launch_pos(slingshot_transform).sub(
                    Vec3::new((birds.len() - i) as f32 * SCALE, SCALE * 2.0, 0.0),
                )),
            ));

        if i == 0 {
            b.insert(CurrentBird);
        }

        i += 1;

        entities.push(b.id());
    }

    entities
}
