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
        Some(mut c) =>
        {
            c.spritexml.add_anim_from_prefix("BF idle dance".to_string(), true, 24).unwrap();
            c.spritexml.set_anim("BF idle dance".to_string(), &mut c.sprite_sheet.sprite).unwrap();
            c.spritexml.apply_offsets(&c.sprite_sheet.sprite, &mut c.sprite_sheet.transform); // apply inital offsets
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

