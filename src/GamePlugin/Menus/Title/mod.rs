use bevy::prelude::*;
use super::super::GameState;
use super::super::Core::SpriteXML;

mod systems;
pub struct TitlePlugin;

impl Plugin for TitlePlugin
{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(|asset_server: Res<AssetServer>, mut epic: ResMut<systems::FunkinAssets>|
                {
                    epic.bumpin = asset_server.load("images/cool.png");
                    epic.yourgf = asset_server.load("images/gfDanceTitle.png");
                    epic.pressenter = asset_server.load("images/titleEnter.png");
                    
                })
                .insert_resource(systems::FunkinAssets::default())
            .add_system(systems::title_init.in_schedule(OnEnter(GameState::Title)))
            .add_system(systems::bye_flash.in_set(OnUpdate(GameState::Title)))
            .add_system(systems::beat_heat.in_set(OnUpdate(GameState::Title)).after(systems::handle_beatstate))
            .add_system(systems::handle_beatstate.in_set(OnUpdate(GameState::Title)))
            .add_system(SpriteXML::tick_animations.in_set(OnUpdate(GameState::Title)))
            .add_system(SpriteXML::tick_animations_3D.in_set(OnUpdate(GameState::Title)));
    }
}