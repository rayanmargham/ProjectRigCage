use bevy::prelude::*;
mod Intro;
mod Title;
use Title::TitlePlugin;
use Intro::IntroPlugin;
pub mod Letters;
pub struct MenusPlugin;

impl Plugin for MenusPlugin
{
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TitlePlugin)
            .add_plugin(IntroPlugin);
    }
}