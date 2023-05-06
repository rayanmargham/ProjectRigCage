use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_sprite3d::Sprite3dParams;
use crate::{GamePlugin::{Gameplay::Conductor::{self, ConductorEvents, SongHandle}, PreLoader, Core::SpriteXML::*, Menus::Letters::{LetterBundle, LetterBundle3D}}};

pub fn intro_init(mut commands: Commands, mut conductor: ResMut<Conductor::Conductor>, audio: Res<Audio>, preloaded: Res<PreLoader::PreloadFunkinAssets>, mut sprite_params: Sprite3dParams)
{
    
    let handle = audio.play(preloaded.freaky.clone()).looped().handle();
    conductor.changeBPM(102.0);
    commands.insert_resource(SongHandle(handle));

    
}

pub fn handle_beatstate(mut conductor: ResMut<Conductor::Conductor>, mut writer: EventWriter<ConductorEvents>, handle: Res<SongHandle>, audio_instances: Res<Assets<AudioInstance>>)
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

