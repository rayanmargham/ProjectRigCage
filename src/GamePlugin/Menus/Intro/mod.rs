use bevy::prelude::*;
use super::super::GameState;

mod systems;
pub struct IntroPlugin;

impl Plugin for IntroPlugin
{
    fn build(&self, app: &mut App) {
        app
            .add_system(systems::intro_init.in_schedule(OnEnter(GameState::Intro)))
            .add_system(systems::update_frame.in_set(OnUpdate(GameState::Intro)))
            .add_system(systems::handle_beatstate.in_set(OnUpdate(GameState::Intro)));
    }
}