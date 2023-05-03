use bevy::prelude::*;
mod Intro;
use Intro::IntroPlugin;

pub struct MenusPlugin;

impl Plugin for MenusPlugin
{
    fn build(&self, app: &mut App) {
        app
            .add_plugin(IntroPlugin);
    }
}