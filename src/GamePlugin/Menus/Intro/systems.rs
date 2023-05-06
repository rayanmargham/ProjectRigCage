use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_sprite3d::Sprite3dParams;
use crate::{GamePlugin::{Gameplay::Conductor::{self, ConductorEvents, SongHandle}, PreLoader, Core::SpriteXML::*, Menus::Letters::LetterBundle}};

pub fn intro_init(mut commands: Commands, mut conductor: ResMut<Conductor::Conductor>, audio: Res<Audio>, preloaded: Res<PreLoader::PreloadFunkinAssets>, mut sprite_params: Sprite3dParams)
{
    
    let handle = audio.play(preloaded.freaky.clone()).handle();
    conductor.changeBPM(102.0);
    commands.insert_resource(SongHandle(handle));

    let bfs = sprite_params.atlases.add(TextureAtlas::new_empty(preloaded.bf.clone(), Vec2::new(8192.0, 4096.0)));
    let Some(bf) = sprite_params.atlases.get_mut(&bfs) else { return };
    let xml = SpriteXMLBundle::new("assets/images/BOYFRIEND.xml".to_string(), &bfs, bf);
    let mut cool = SpriteXMLBundle3D::new("assets/images/BOYFRIEND.xml".to_string(), &bfs, &mut sprite_params, true).unwrap();
    
    match xml {
        Some(mut c) =>
        {
            c.spritexml.add_anim_from_prefix("BF idle".to_string(), false, 24).unwrap();
            cool.spritexml.add_anim_from_prefix("BF idle".to_string(), true, 24).unwrap();
            c.spritexml.set_anim("BF idle".to_string(), &mut c.sprite_sheet.sprite, true).unwrap();
            cool.spritexml.set_anim("BF idle".to_string(), &mut cool.sprite_sheet.params, true).unwrap();
            c.spritexml.apply_offsets(&c.sprite_sheet.sprite, &mut c.sprite_sheet.transform); // apply inital offsets
            cool.spritexml.apply_offsets(&cool.sprite_sheet.params, &mut cool.sprite_sheet.pbr.transform);
            cool.sprite_sheet.pbr.transform.translation.z = -5.0;
            c.sprite_sheet.transform.translation.x += 400.0;
            LetterBundle::new("friday".to_string(), &mut commands, &preloaded, &mut sprite_params.atlases, Transform::from_xyz(-50.0, 0.0, 1.0));
            commands.spawn(c);
            commands.spawn(cool);
        },
        None =>
        {
            error!("DIDN'T WORK BOZO!!!");
        }

    }
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

pub fn bf_idle(mut reader: EventReader<ConductorEvents>, mut query: Query<&mut SpriteXML, With<SpriteXML>>)
{
    for ev in reader.iter()
    {
        match ev
        {
            ConductorEvents::BeatHit =>
            {
                for mut xml in query.iter_mut()
                {
                    xml.reset_anim_idx();
                }
            }
            _ =>
            {

            }
        }
    }
}