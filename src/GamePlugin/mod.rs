use bevy::prelude::*;
pub mod Gameplay;
pub mod GameStates;
pub mod PreLoader;
pub mod Menus;
pub mod Core;
use Gameplay::Conductor;

use self::{GameStates::GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Conductor::Conductor>()
            .add_event::<Conductor::ConductorEvent>()
            .add_state::<GameState>()
            .add_plugin(PreLoader::PreloaderPlugin)
            .add_plugin(Menus::MenusPlugin);
            
    }
}