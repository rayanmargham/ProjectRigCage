use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::{GamePlugin::{Gameplay::Conductor::{self, ConductorEvent, SongHandle}, PreLoader}};

pub fn intro_init(mut commands: Commands, mut conductor: ResMut<Conductor::Conductor>, audio: Res<Audio>, preloaded: Res<PreLoader::PreloadFunkinAssets>, mut texture_atlases: ResMut<Assets<TextureAtlas>>)
{
    
    let handle = audio.play(preloaded.freaky.clone()).handle();
    conductor.changeBPM(102.0);
    commands.insert_resource(SongHandle(handle));
}

pub fn handle_beatstate(mut conductor: ResMut<Conductor::Conductor>, mut writer: EventWriter<ConductorEvent>, handle: Res<SongHandle>, audio_instances: Res<Assets<AudioInstance>>)
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