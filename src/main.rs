#![allow(non_snake_case)]
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

// mod it in lol
mod Conductor;
mod Song;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .init_resource::<Conductor::Conductor>()
        .add_event::<Conductor::ConductorEvent>()
        .add_startup_system(spawn_world)
        .add_startup_system(test_conductor)
        .add_system(update_songpos)
        .run();
}

fn spawn_world(mut commands: Commands, mut meshs: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
    commands.spawn(PbrBundle {
        mesh: meshs.add(shape::Cube { size: 1.0 }.into()),
        transform: Transform::from_xyz(0.0, 0.0, -5.0),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        ..default()
    });
    commands.spawn(Camera3dBundle::default());
    let font = asset_server.load("fonts/pixel.otf");

    commands.spawn(TextBundle::from_section("Hello World!", TextStyle { font: font.clone(), font_size: 50.0, color: Color::WHITE })
        .with_style(
            Style
            {
                position_type: PositionType::Absolute,
                position: UiRect
                {
                    top: ,
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }
        ));

}

fn test_conductor(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>, mut conduct: ResMut<Conductor::Conductor>)
{
    let handle = audio.play(asset_server.load("audio/Inst.ogg")).handle();
    conduct.song = Some(Song::loadJson("assets/data/monster-hard.json").unwrap().song);
    let epic = conduct.song.as_ref().unwrap().bpm;
    conduct.changeBPM(epic);
    conduct.mapBPMChanges();
    commands.insert_resource(Conductor::SongHandle(handle));
}

fn update_songpos(handle: Res<Conductor::SongHandle>, audio_instances: ResMut<Assets<AudioInstance>>, mut conduct: ResMut<Conductor::Conductor>, mut writer: EventWriter<Conductor::ConductorEvent>)
{
    if let Some(instance) = audio_instances.get(&handle.0)
    {
        match instance.state()
        {
            PlaybackState::Playing {..} =>
            {
                conduct.songPos = (instance.state().position().unwrap() as f32) * 1000.0;
                conduct.update_beatstate(writer);
            }
            _ =>
            {

            }
        }
        
    }
}
