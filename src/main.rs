#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{app::App, prelude::bevy_main};
use strangrybirds::StrangryBirdsGame;

#[bevy_main]
fn main() {
    App::new().add_plugins(StrangryBirdsGame).run();
}
