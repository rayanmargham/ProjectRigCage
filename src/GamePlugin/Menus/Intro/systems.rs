use std::time::Duration;

use bevy::{prelude::*};
use rand::*;
use bevy_kira_audio::prelude::*;
use bevy_sprite3d::{Sprite3dParams, Sprite3d};
use crate::{GamePlugin::{Gameplay::Conductor::{self, ConductorEvents, SongHandle}, PreLoader, Core::SpriteXML::*, Menus::Letters::{LetterBundle, LetterBundle3D}, GameStates::GameState}};

#[derive(Component)]
pub struct FreakyText;

#[derive(Resource, Default)]
pub struct Branding(pub Handle<Image>);

#[derive(Resource)]
pub struct wackymuffin
{
    text1: String,
    text2: String
}
pub fn intro_init(mut commands: Commands, mut conductor: ResMut<Conductor::Conductor>, audio: Res<Audio>, preloaded: Res<PreLoader::PreloadFunkinAssets>, mut atlases: ResMut<Assets<TextureAtlas>>)
{
    
    let handle = audio.play(preloaded.freaky.clone()).looped().with_volume(0.7).fade_in(AudioTween::new(Duration::from_secs_f64(0.7) ,AudioEasing::Linear)).handle();
    conductor.changeBPM(102.0);
    commands.insert_resource(SongHandle(handle));
    atlases.add(TextureAtlas::new_empty(preloaded.alphabet.clone(), Vec2::new(1024.0, 695.0)));
    let collection = read_epic_text("assets/data/intro/introtext.txt");
    let mut rng = thread_rng();
    let rand = rng.gen_range(0..collection.len() - 1);
    commands.insert_resource(wackymuffin
    {
        text1: collection[rand][0].clone(),
        text2: collection[rand][1].clone()
    });
    
}

pub fn handle_beatstate(mut conductor: ResMut<Conductor::Conductor>, writer: EventWriter<ConductorEvents>, handle: Res<SongHandle>, audio_instances: Res<Assets<AudioInstance>>)
{
    conductor.update_beatstate(writer);
    if let Some(instance) = audio_instances.get(&handle.0)
    {
        match instance.state()
        {
            PlaybackState::Playing { position } =>
            {
                conductor.songPos = position * 1000.0;
            }
            _ =>
            {

            }
        }
    }
}

pub fn read_epic_text(path: &str) -> Vec<Vec<String>>
{
    let cool = std::fs::read_to_string(path).unwrap();
    let mut swagger: Vec<Vec<String>> = Vec::new();
    for i in cool.lines()
    {
        swagger.push(i.split("--").map(String::from).collect());
    }
    return swagger;
}
pub fn freakybeathit(mut next_state: ResMut<NextState<GameState>>, ninjapuffin: Res<wackymuffin>, brand: Res<Branding>, conduct: Res<Conductor::Conductor>, mut reader: EventReader<ConductorEvents>, mut commands: Commands, query: Query<Entity, With<FreakyText>>, mut sprite_params: Sprite3dParams, preloaded: Res<PreLoader::PreloadFunkinAssets>)
{
    for ev in reader.iter()
    {
        match ev {
            ConductorEvents::BeatHit =>
            {
                match conduct.curBeat {
                    1 =>
                    {
                        let ent = LetterBundle3D::new("Rust Rewrite By".to_string(), &mut commands, &preloaded, &mut sprite_params, Transform::from_xyz(-3.3, 0.8, -10.0), false).unwrap();
                        
                        commands.entity(ent).insert(FreakyText);
                    },
                    3 =>
                    {
                        let ent = LetterBundle3D::new("Linux".to_string(), &mut commands, &preloaded, &mut sprite_params, Transform::from_xyz(-1.0, 0.0, -10.0), false).unwrap();
                        

                        commands.entity(ent).insert(FreakyText);
                    },
                    4 =>
                    {
                        for bye in query.iter()
                        {
                            commands.entity(bye).despawn_recursive();
                        }
                    },
                    5 =>
                    {
                        let ent = LetterBundle3D::new("Built With".to_string(), &mut commands, &preloaded, &mut sprite_params, Transform::from_xyz(-2.1, 2.0, -10.0), false).unwrap();

                        commands.entity(ent).insert(FreakyText);
                    },
                    7 =>
                    {
                        commands.spawn(Sprite3d
                        {
                            image: brand.0.clone(),
                            transform: Transform::from_xyz(0.0, 0.0, -10.0),
                            partial_alpha: true,
                            unlit: true,
                            ..default()
                        }.bundle(&mut sprite_params)).insert(FreakyText);
                    },
                    8 =>
                    {
                        for bye in query.iter()
                        {
                            // bozo is gone lmaoo
                            commands.entity(bye).despawn_recursive();
                        }
                    },
                    9 =>
                    {
                        let ent = LetterBundle3D::new(ninjapuffin.text1.clone(), &mut commands, &preloaded, &mut sprite_params, Transform::from_xyz(0.0, 0.8, -10.0), true).unwrap();

                        commands.entity(ent).insert(FreakyText);
                    },
                    11 =>
                    {
                        let ent = LetterBundle3D::new(ninjapuffin.text2.clone(), &mut commands, &preloaded, &mut sprite_params, Transform::from_xyz(0.0, 0.0, -10.0), true).unwrap();

                        commands.entity(ent).insert(FreakyText);
                    },
                    12 =>
                    {
                        for bye in query.iter()
                        {
                            // bozo is gone lmaoo
                            commands.entity(bye).despawn_recursive();
                        }
                    },
                    13 =>
                    {
                        let ent = LetterBundle3D::new("Friday".to_string(), &mut commands, &preloaded, &mut sprite_params, Transform::from_xyz(-1.39, 1.2, -10.0).with_scale(Vec3::new(1.1, 1.1, 1.0)), false).unwrap();

                        commands.entity(ent).insert(FreakyText);
                    },
                    14 =>
                    {
                        let ent = LetterBundle3D::new("Night".to_string(), &mut commands, &preloaded, &mut sprite_params, Transform::from_xyz(0.0, 0.3, -10.0), true).unwrap();

                        commands.entity(ent).insert(FreakyText);
                    },
                    15 =>
                    {
                        let ent = LetterBundle3D::new("Funkin".to_string(), &mut commands, &preloaded, &mut sprite_params, Transform::from_xyz(-1.25, -0.6, -10.0), false).unwrap();

                        commands.entity(ent).insert(FreakyText);
                    },
                    16 =>
                    {
                        for bye in query.iter()
                        {
                            // bozo is gone lmaoo
                            commands.entity(bye).despawn_recursive();
                        }
                        next_state.set(GameState::Title);
                        
                    }

                    _ =>
                    {

                    }
                }
            },
            _ =>
            {

            }
            
        }
    }
}