use avian2d::prelude::{Collider, RigidBody};
use bevy::{ecs::component::Component, math::Vec2, prelude::Resource};

use crate::common::{Despawn, Id, SCALE};

#[derive(Clone, Component)]
#[require(Despawn)]
pub struct Object {
    pub id: Id,
    pub asset_path: String,
    pub size: Vec2,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

#[derive(Resource)]
pub struct Objects(pub Vec<Object>);

impl Objects {
    pub fn add(&mut self, asset_path: &str, size: Vec2, collider: Collider, rigid_body: RigidBody) {
        self.0.push(Object {
            id: self.0.len() + 1, // add one because id 0 is air
            asset_path: asset_path.into(),
            size,
            collider,
            rigid_body,
        });
    }
    
    pub fn add_landscape(&mut self, asset_path: &str, size: Vec2, collider: Collider) {
        self.add(asset_path, size, collider, RigidBody::Static);
    }

    pub fn add_movable(&mut self, asset_path: &str, size: Vec2, collider: Collider) {
        self.add(asset_path, size, collider, RigidBody::Dynamic);
    }

    pub fn add_default(&mut self, asset_path: &str) {
        self.add_movable(asset_path, Vec2::ONE, Collider::rectangle(SCALE, SCALE));
    }

    pub fn get(&self, id: Id) -> Option<&Object> {
        self.0.iter().find(|object| id == object.id)
    }
}

impl Default for Objects {
    fn default() -> Self {
        let mut objects = Self(vec![]);
        objects.add_landscape("debug.png", (1.0, 1.0).into(), Collider::rectangle(SCALE, SCALE));

        objects
    }
}
