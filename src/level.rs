use bevy::{
    asset::AssetServer,
    ecs::{
        entity::Entity,
        message::{MessageReader, MessageWriter},
        query::With,
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    math::Vec2,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

use crate::{
    birds::{Bird, spawn_birds},
    common::{Despawn, Id, LevelDespawnEvent, LevelSpawnEvent, SCALE, Score},
    object::Objects,
    slingshot::spawn_slingshot,
};

pub struct Level {
    pub id: Id,
    pub next: Option<Id>,
    pub slingshot: Vec2,
    pub birds: Vec<Bird>,
    pub objects: Vec<(f32, f32, Id)>,
}

#[derive(Resource)]
pub struct Levels(pub Vec<Level>);

impl Levels {
    pub fn get(&self, id: Id) -> Option<&Level> {
        self.0.iter().find(|level| id == level.id)
    }

    // parse tiles into objects with positions
    pub fn add(
        &mut self,
        next: Option<Id>,
        slingshot: Vec2,
        birds: Vec<Bird>,
        objects: Vec<(f32, f32, Id)>,
        tiles: Vec<Vec<Id>>,
    ) {
        let mut level = Level {
            id: self.0.len(),
            next,
            slingshot,
            birds,
            objects,
        };

        let mut y = tiles.len() as f32;
        for row in tiles {
            let mut x = 0.0;
            for column in row {
                // id 0 is air
                if column != 0 {
                    level.objects.push((x, y, column));
                }

                x += 1.0;
            }

            y -= 1.0;
        }

        self.0.push(level);
    }
}

impl Default for Levels {
    fn default() -> Self {
        let mut levels = Self(vec![]);
        levels.add(
            Some(1),
            (-5.0, 0.0).into(),
            vec![Bird::Strang, Bird::Strang],
            vec![(10.0, 10.0, 1)],
            vec![
                vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );

        levels
    }
}

pub fn spawn_level(
    mut reader: MessageReader<LevelSpawnEvent>,
    // mut writer: MessageWriter<LevelDespawnEvent>,
    mut commands: Commands,
    levels: Res<Levels>,
    objects: Res<Objects>,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
) {
    for event in reader.read() {
        // writer.write(LevelDespawnEvent);
        score.0 = 0;

        if let Some(level) = levels.get(event.0) {
            let slingshot_transform =
                spawn_slingshot(level.slingshot, commands.reborrow(), &asset_server);
            spawn_birds(
                &level.birds,
                &slingshot_transform,
                commands.reborrow(),
                &asset_server,
            );

            for (x, y, id) in &level.objects {
                if let Some(object) = objects.get(*id) {
                    commands.spawn((
                        object.clone(),
                        Sprite {
                            image: asset_server.load(object.asset_path.clone()),
                            custom_size: Some(object.size * SCALE),
                            ..default()
                        },
                        object.rigid_body,
                        object.collider.clone(),
                        Transform::from_xyz(
                            (*x - object.size.x / 2.0) * SCALE,
                            (*y - object.size.y / 2.0) * SCALE,
                            0.0,
                        ),
                    ));
                }
            }
        }
    }
}

pub fn despawn_level(
    mut reader: MessageReader<LevelDespawnEvent>,
    mut commands: Commands,
    entities: Query<Entity, With<Despawn>>,
) {
    for _ in reader.read() {
        for entity in entities {
            commands.entity(entity).try_despawn();
        }
    }
}
