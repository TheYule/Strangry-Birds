pub mod birds;
pub mod common;
pub mod level;
pub mod object;
pub mod slingshot;
pub mod ui;

use avian2d::{PhysicsPlugins, dynamics::integrator::Gravity, math::Vector};
use bevy::{
    DefaultPlugins,
    app::{App, Plugin, PluginGroup, Startup, Update},
    asset::AssetServer,
    camera::{Camera2d, ClearColor},
    color::Color,
    ecs::{
        message::MessageWriter,
        query::With,
        system::{Commands, Query, Res},
    },
    text::{FontSize, FontSource, TextColor, TextFont},
    ui::{Node, PositionType, UiRect, Val, widget::Text},
    utils::default,
    window::{PresentMode, Window, WindowPlugin},
};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_pannzoom::{PanNZoomCam, PanNZoomPlugin};

use crate::{
    common::{LevelDespawnEvent, LevelSpawnEvent, SCALE, Score, SlingshotLaunchEvent},
    level::{Levels, despawn_level, spawn_level},
    object::Objects,
    slingshot::slingshot_launch,
    ui::ScoreText,
};

pub struct StrangryBirdsGame;

impl Plugin for StrangryBirdsGame {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    title: "Strangry Birds".into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            PanNZoomPlugin,
        ))
        .insert_resource(ClearColor(Color::hsv(0.0, 0.0, 0.2039)))
        .insert_resource(Gravity(SCALE * Vector::NEG_Y))
        .init_resource::<Score>()
        .init_resource::<Objects>()
        .init_resource::<Levels>()
        .add_message::<LevelSpawnEvent>()
        .add_message::<LevelDespawnEvent>()
        .add_message::<SlingshotLaunchEvent>()
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (spawn_level, despawn_level, update_score, slingshot_launch),
        );
    }
}

fn startup(
    mut writer: MessageWriter<LevelSpawnEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2d,
        PanNZoomCam {
            min_scale: 1.0,
            max_scale: 5.0,
            min_x: -1000.0,
            max_x: 10000.0,
            min_y: -200.0,
            max_y: 5000.0,
            ..default()
        },
    ));

    commands.spawn((
        Node {
            margin: UiRect::all(Val::Px(10.0)),
            position_type: PositionType::Absolute,
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            ..default()
        },
        Text::new("Score"),
        TextColor(Color::hsv(0.0, 0.0, 0.933)),
        TextFont {
            font: FontSource::Handle(asset_server.load("ArchivoBlack-Regular.ttf")),
            font_size: FontSize::Px(28.0),
            ..default()
        },
        ScoreText,
    ));

    writer.write(LevelSpawnEvent(0));
}

fn update_score(query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut text in query {
        text.0 = score.0.to_string();
    }
}
