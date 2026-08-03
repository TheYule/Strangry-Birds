use bevy::ecs::{component::Component, entity::Entity, message::Message, resource::Resource};

pub const SCALE: f32 = 64.0;

pub type Id = usize;

#[derive(Default, Component)]
pub struct Despawn;

#[derive(Default, Resource)]
pub struct Score(pub usize);

#[derive(Message)]
pub struct LevelSpawnEvent(pub Id);

#[derive(Message)]
pub struct LevelDespawnEvent;

#[derive(Message)]
pub struct SlingshotLaunchEvent {
    pub slingshot: Entity,
}
