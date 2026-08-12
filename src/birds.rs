use std::ops::Sub;

use avian2d::prelude::{Collider, LinearDamping, RigidBody};
use bevy::{
    ecs::{VariantDefaults, entity::Entity, system::Commands},
    math::Vec3,
    prelude::Component,
    scene::{CommandsSceneExt, Scene, bsn, template_value},
    sprite::Sprite,
    transform::components::Transform,
};

use crate::{
    common::{Despawn, SCALE},
    slingshot::Slingshot,
};

#[derive(Clone, Component, Copy, Default, VariantDefaults)]
#[require(Despawn)]
pub enum Bird {
    #[default]
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

    pub fn spawn_ghost(&self) -> impl Scene {
        bsn! {
            Sprite {
                image: {self.asset_path().to_string()},
            }
            template_value(self.clone())
        }
    }

    fn collider(&self) -> Collider {
        Collider::circle(self.size())
    }

    pub fn spawn(&self) -> impl Scene {
        bsn! {
            {self.spawn_ghost()}
            template_value(RigidBody::Dynamic)
            template_value(self.collider())
            LinearDamping(0.2)
        }
    }
}

#[derive(Component)]
pub struct CurrentBird;

#[derive(Clone, Component, Copy, Debug, Default, Hash)]
pub struct BirdOrder(pub usize);

pub fn spawn_birds(
    birds: &Vec<Bird>,
    slingshot_transform: &Transform,
    mut commands: Commands,
) -> Vec<Entity> {
    let mut entities = vec![];
    let mut i = 0;

    for bird in birds {
        let mut b = commands.spawn_scene(bsn! {
            {bird.spawn_ghost()}
            BirdOrder(i)
            template_value(Transform::from_translation(Slingshot::launch_pos(slingshot_transform).sub(
                Vec3::new((birds.len() - i) as f32 * SCALE, SCALE * 2.0, 0.0),
            )))
        });

        if i == 0 {
            b.insert(CurrentBird);
        }

        i += 1;

        entities.push(b.id());
    }

    entities
}
