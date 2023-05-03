#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::*;
// mod it in lol
mod GamePlugin;
mod Song;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(GamePlugin::GamePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
