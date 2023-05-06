use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_sprite3d::Sprite3dParams;
use crate::{GamePlugin::{Gameplay::Conductor::{self, ConductorEvents, SongHandle}, PreLoader, Core::SpriteXML::*, Menus::Letters::{LetterBundle, LetterBundle3D}}};

#[derive(Component)]
pub struct FreakyText;
pub fn intro_init(mut commands: Commands, mut conductor: ResMut<Conductor::Conductor>, audio: Res<Audio>, preloaded: Res<PreLoader::PreloadFunkinAssets>, mut atlases: ResMut<Assets<TextureAtlas>>)
{
    
    let handle = audio.play(preloaded.freaky.clone()).looped().with_volume(0.7).fade_in(AudioTween::new(Duration::from_secs_f64(0.7) ,AudioEasing::Linear)).handle();
    conductor.changeBPM(102.0);
    commands.insert_resource(SongHandle(handle));
    atlases.add(TextureAtlas::new_empty(preloaded.alphabet.clone(), Vec2::new(1024.0, 695.0)));

    
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
                conductor.songPos = (position * 1000.0) as f32;
            }
            _ =>
            {

            }
        }
    }
}

pub fn freakybeathit(conduct: Res<Conductor::Conductor>, mut reader: EventReader<ConductorEvents>, mut commands: Commands, query: Query<Entity, With<FreakyText>>, mut sprite_params: Sprite3dParams, preloaded: Res<PreLoader::PreloadFunkinAssets>)
{
    for ev in reader.iter()
    {
        match ev {
            ConductorEvents::BeatHit =>
            {
                match conduct.curBeat {
                    1 =>
                    {
                        let ent = LetterBundle3D::new("Rust Rewrite By".to_string(), &mut commands, &preloaded, &mut sprite_params, Transform::from_xyz(-3.3, 1.0, -10.0)).unwrap();
                        
                        commands.entity(ent).insert(FreakyText);
                    },
                    3 =>
                    {
                        let ent = LetterBundle3D::new("Linux".to_string(), &mut commands, &preloaded, &mut sprite_params, Transform::from_xyz(-1.0, 0.0, -10.0)).unwrap();
                        let ent2 = LetterBundle3D::new("Leather".to_string(), &mut commands, &preloaded, &mut sprite_params, Transform::from_xyz(-1.45, -1.0, -10.0)).unwrap();

                        commands.entity(ent).insert(FreakyText);
                        commands.entity(ent2).insert(FreakyText);
                    },
                    4 =>
                    {
                        for bye in query.iter()
                        {
                            commands.entity(bye).despawn_recursive();
                        }
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