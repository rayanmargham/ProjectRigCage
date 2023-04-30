use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
pub mod Gameplay;
pub mod GameStates;
pub mod PreLoader;
use Gameplay::Conductor;

use self::{GameStates::GameState, PreLoader::{PreloadClockAsset, PreloadFunkinAssets}};

pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Conductor::Conductor>()
            .add_event::<Conductor::ConductorEvent>()
            .add_state::<GameState>()
            .add_loading_state(LoadingState::new(GameState::ClockAsset).continue_to_state(GameState::Preload))
            .add_loading_state(LoadingState::new(GameState::Preload))
            .add_collection_to_loading_state::<_, PreloadClockAsset>(GameState::ClockAsset)
            .add_collection_to_loading_state::<_, PreloadFunkinAssets>(GameState::Preload)
            .add_system(PreLoader::PreLoaderInit.in_schedule(OnEnter(GameState::Preload)))
            .add_system(PreLoader::TickClockTimer.in_set(OnUpdate(GameState::Preload)))
            .add_system(PreLoader::Clock.in_set(OnUpdate(GameState::Preload)))
            .add_system(PreLoader::DebugRot.in_set(OnUpdate(GameState::Preload)));
            
    }
}