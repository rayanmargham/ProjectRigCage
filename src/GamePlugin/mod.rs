use bevy::prelude::*;
pub mod Core;
pub mod GameStates;
pub mod Gameplay;
pub mod Menus;
pub mod PreLoader;
use bevy_sprite3d::Sprite3dPlugin;
use Gameplay::Conductor;

use self::{GameStates::GameState, Gameplay::GameplayPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Conductor::Conductor>()
            .add_event::<Conductor::ConductorEvents>()
            .add_state::<GameState>()
            .add_plugin(Sprite3dPlugin)
            .add_plugin(PreLoader::PreloaderPlugin)
            .add_plugin(GameplayPlugin)
            .add_plugin(Menus::MenusPlugin);
    }
}
