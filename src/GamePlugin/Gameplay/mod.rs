pub mod Conductor;

use bevy::prelude::*;
use crate::GamePlugin::GameState;
use crate::GamePlugin::Core::SpriteXML;

mod systems;
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin
{
    fn build(&self, app: &mut App) {
        app
            .add_system(SpriteXML::tick_animations.in_schedule(OnEnter(GameState::Gameplay)))
            .add_system(SpriteXML::tick_animations_3D.in_set(OnUpdate(GameState::Gameplay)));
    }
}