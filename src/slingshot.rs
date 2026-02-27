use std::ops::{Add, Mul, Sub};

use avian2d::prelude::LinearVelocity;
use bevy::{
    asset::AssetServer,
    ecs::{
        entity::Entity,
        event::EntityEvent,
        message::{MessageReader, MessageWriter},
        observer::On,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    math::{Vec2, Vec3},
    picking::{
        Pickable,
        events::{Drag, DragEnd, Pointer},
    },
    prelude::Component,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

use crate::{
    birds::{Bird, CurrentBird},
    common::{Despawn, SCALE, SlingshotLaunchEvent},
};

#[derive(Default, Component)]
#[require(Despawn)]
pub struct Slingshot {
    pub distance: Vec2,
}

pub fn spawn_slingshot(pos: Vec2, mut commands: Commands, asset_server: &AssetServer) {
    commands
        .spawn((
            Slingshot::default(),
            Sprite {
                image: asset_server.load("slingshot.png"),
                custom_size: Some((SCALE, SCALE * 2.0).into()),
                ..default()
            },
            Transform::from_translation(
                pos.sub(Vec2::new(0.5, 1.0))
                    .mul(SCALE)
                    .extend(1.0),
            ),
            Pickable::default(),
        ))
        .observe(drag)
        .observe(drag_end);
}

fn drag(trigger: On<Pointer<Drag>>, mut query: Query<&mut Slingshot>) {
    if let Ok(mut slingshot) = query.get_mut(trigger.event_target()) {
        let event = trigger.event();

        slingshot.distance = event.distance.clamp_length_max(SCALE * 2.0);
        if slingshot.distance.length() < SCALE / 2.0 {
            slingshot.distance = Vec2::ZERO;
        }
    }
}

fn drag_end(
    trigger: On<Pointer<DragEnd>>,
    query: Query<Entity, With<CurrentBird>>,
    mut writer: MessageWriter<SlingshotLaunchEvent>,
) {
    let slingshot_entity = trigger.event_target();

    if let Ok(bird_entity) = query.single() {
        writer.write(SlingshotLaunchEvent {
            slingshot: slingshot_entity,
            bird: bird_entity,
        });
    }
}

pub fn slingshot_launch(
    mut reader: MessageReader<SlingshotLaunchEvent>,
    mut commands: Commands,
    slingshot_query: Query<(&Sprite, &Transform, &mut Slingshot), Without<CurrentBird>>,
    bird_query: Query<(Entity, &Bird), (With<CurrentBird>, Without<Slingshot>)>,
    asset_server: Res<AssetServer>,
) {
    for event in reader.read() {
        if let Ok((slingshot_sprite, slingshot_transform, slingshot)) =
            slingshot_query.get(event.slingshot)
        {
            if let Ok((bird_entity, bird)) = bird_query.get(event.bird) {
                // Despawn ghost bird and spawn real bird
                commands.entity(bird_entity).despawn();
                commands.spawn((
                    bird.spawn(&asset_server),
                    LinearVelocity(slingshot.distance * -1.0),
                    Transform::from_translation(slingshot_transform.translation.add(Vec3::new(
                        0.0,
                        slingshot_sprite.custom_size.unwrap_or_default().y / 2.0,
                        0.0,
                    ))),
                ));
            }
        }
    }
}
