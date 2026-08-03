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
        events::{Drag, DragEnd, DragStart, Pointer},
    },
    prelude::Component,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};
use bevy_pannzoom::PanNZoomCam;

use crate::{
    birds::{Bird, CurrentBird},
    common::{Despawn, SCALE, SlingshotLaunchEvent},
};

#[derive(Default, Component)]
#[require(Despawn)]
pub struct Slingshot {
    pub distance: Vec2,
}

impl Slingshot {
    pub fn size() -> Vec2 {
        Vec2::new(SCALE, SCALE * 2.0)
    }

    pub fn launch_pos(transform: &Transform) -> Vec3 {
        transform
            .translation
            .add(Vec3::new(0.0, Self::size().y / 2.0, 0.0))
    }
}

pub fn spawn_slingshot(pos: Vec2, mut commands: Commands, asset_server: &AssetServer) -> Transform {
    let transform =
        Transform::from_translation(pos.sub(Vec2::new(0.5, 1.0)).mul(SCALE).extend(1.0));

    commands
        .spawn((
            Slingshot::default(),
            Sprite {
                image: asset_server.load("slingshot.png"),
                custom_size: Some(Slingshot::size()),
                ..default()
            },
            transform,
            Pickable::default(),
        ))
        .observe(drag_start)
        .observe(drag)
        .observe(drag_end);

    transform
}

fn drag_start(_: On<Pointer<DragStart>>, query: Query<&mut PanNZoomCam>) {
    for mut pancam in query {
        pancam.enabled = false;
    }
}

fn drag(
    trigger: On<Pointer<Drag>>,
    mut slingshot_query: Query<(&Transform, &mut Slingshot), Without<CurrentBird>>,
    bird_query: Query<(&mut Transform, &Bird), (With<CurrentBird>, Without<Slingshot>)>,
) {
    if let Ok((slingshot_transform, mut slingshot)) =
        slingshot_query.get_mut(trigger.event_target())
    {
        let event = trigger.event();

        slingshot.distance = event.distance.clamp_length_max(SCALE * 2.0);
        if slingshot.distance.length() < SCALE / 2.0 {
            slingshot.distance = Vec2::ZERO;
        }

        for (mut bird_transform, bird) in bird_query {
            bird_transform.translation = slingshot_transform
                .translation
                .sub(slingshot.distance.mul(Vec2::new(-1.0, 1.0)).extend(0.0))
                .sub(Vec3::new(bird.size() / 2.0, -bird.size() / 2.0, 0.0));
        }

        println!("{:?}", slingshot.distance);
    }
}

fn drag_end(
    trigger: On<Pointer<DragEnd>>,
    query: Query<&mut PanNZoomCam, (Without<Slingshot>, Without<CurrentBird>)>,
    mut writer: MessageWriter<SlingshotLaunchEvent>,
) {
    for mut pancam in query {
        pancam.enabled = true;
    }

    // TODO: prevent triggering event when `slingshot.distance` is zero
    writer.write(SlingshotLaunchEvent {
        slingshot: trigger.event_target(),
    });
}

pub fn slingshot_launch(
    mut reader: MessageReader<SlingshotLaunchEvent>,
    mut commands: Commands,
    slingshot_query: Query<(&Transform, &mut Slingshot), Without<CurrentBird>>,
    bird_query: Query<(Entity, &Bird), (With<CurrentBird>, Without<Slingshot>)>,
    asset_server: Res<AssetServer>,
) {
    for event in reader.read() {
        println!("Launch!");

        if let Ok((slingshot_transform, slingshot)) = slingshot_query.get(event.slingshot) {
            // TODO: move to `drag_end`
            if slingshot.distance.length() == 0.0 {
                break;
            }

            for (bird_entity, bird) in bird_query {
                // Despawn ghost bird and spawn real bird
                commands.entity(bird_entity).despawn();
                commands.spawn((
                    bird.spawn(&asset_server),
                    LinearVelocity(slingshot.distance * Vec2::new(-5.0, 5.0)),
                    Transform::from_translation(Slingshot::launch_pos(slingshot_transform)),
                ));
            }
        }
    }
}
