pub mod Conductor;

use crate::GamePlugin::Core::SpriteXML;
use crate::GamePlugin::GameState;
use bevy::prelude::*;

mod systems;
#[derive(Resource)]
pub struct PlayStateInformation
{
    pub song: String,
    pub startedcountdown: bool
}
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(SpriteXML::tick_animations.in_set(OnUpdate(GameState::Gameplay)))
            .add_system(systems::setup_state.in_schedule(OnEnter(GameState::Gameplay)))
            .add_system(systems::countdown.in_set(OnUpdate(GameState::Gameplay)))
            .add_system(SpriteXML::tick_animations_3D.in_set(OnUpdate(GameState::Gameplay)));
    }
}
