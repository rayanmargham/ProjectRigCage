use super::super::Core::SpriteXML;
use super::super::GameState;
use bevy::prelude::*;

mod systems;
pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::intro_init.in_schedule(OnEnter(GameState::Intro)))
            .add_system(SpriteXML::tick_animations.in_set(OnUpdate(GameState::Intro)))
            .add_system(SpriteXML::tick_animations_3D.in_set(OnUpdate(GameState::Intro)))
            .add_system(
                systems::freakybeathit
                    .in_set(OnUpdate(GameState::Intro))
                    .after(systems::handle_beatstate),
            )
            .add_startup_system(
                |asset_server: Res<AssetServer>, mut branding: ResMut<systems::Branding>| {
                    asset_server.mark_unused_assets();
                    asset_server.free_unused_assets();
                    branding.0 = asset_server.load("images/bevy.png");
                },
            )
            .insert_resource(systems::Branding::default())
            .add_system(systems::handle_beatstate.in_set(OnUpdate(GameState::Intro)));
    }
}
