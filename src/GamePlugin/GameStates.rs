use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState
{
    #[default]
    // If you don't want a state you can easily just remove it from the enum and change NextState in your System to the State you want.
    ClockAsset, // State to load the clock asset
    Preload, // Load Assets at the Start
    Splash, // Play a lil Splash Animation (Can be Disabled in Config)
    Intro, // Funkin' Intro
    Title, // Gettin' Freaky on a Friday Night Yeah~
    MainMenu, // Main Menu
    StoryMode, // Story Mode Menu
    RunTimeLoader, // Loads Assets just before you enter the GamePlay State
    CutSceneState, // Loads a cutscene, Audio and Video are seperate due to limiations with Bevy Video
    Gameplay, // Actual Game, Beep Bops, Hot Ass Girlfriend on da Speaker and Notes flyin

}