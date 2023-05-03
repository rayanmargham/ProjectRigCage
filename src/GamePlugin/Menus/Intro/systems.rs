use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::{GamePlugin::{Gameplay::Conductor::{self, ConductorEvent, SongHandle}, PreLoader, Core::SpriteXML::{SpriteXMLBundle, SpriteXML}}};

pub fn intro_init(mut commands: Commands, mut conductor: ResMut<Conductor::Conductor>, audio: Res<Audio>, preloaded: Res<PreLoader::PreloadFunkinAssets>, mut texture_atlases: ResMut<Assets<TextureAtlas>>)
{
    
    let handle = audio.play(preloaded.freaky.clone()).handle();
    conductor.changeBPM(102.0);
    commands.insert_resource(SongHandle(handle));

    let bfs = texture_atlases.add(TextureAtlas::new_empty(preloaded.bf.clone(), Vec2::new(8192.0, 4096.0)));
    let Some(bf) = texture_atlases.get_mut(&bfs) else { return };
    let xml = SpriteXMLBundle::new("assets/images/BOYFRIEND.xml".to_string(), &bfs, bf);
    match xml {
        Some(c) =>
        {
            commands.spawn(c);
        },
        None =>
        {
            error!("DIDN'T WORK BOZO!!!");
        }

    }
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

pub fn update_frame(mut query: Query<(&mut SpriteXML, &mut TextureAtlasSprite, &mut Transform), With<SpriteXML>>)
{
    for (mut xml, mut sprite, mut trans) in query.iter_mut()
    {
        xml.get_next_frame(&mut sprite, &mut trans);
    }
}