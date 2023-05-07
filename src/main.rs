#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
use bevy::{prelude::*, render::render_resource::{SamplerDescriptor, AddressMode}};

use bevy_kira_audio::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::*;
// mod it in lol
mod GamePlugin;
mod Song;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: SamplerDescriptor {
              address_mode_u: AddressMode::ClampToBorder,
              address_mode_v: AddressMode::ClampToBorder,
              address_mode_w: AddressMode::ClampToBorder,
              ..Default::default()
            },
          }))
        .add_plugin(AudioPlugin)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(GamePlugin::GamePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
