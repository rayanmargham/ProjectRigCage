use std::time::Duration;

use bevy::prelude::*;
use bevy_sprite3d::{Sprite3dParams, AtlasSprite3dComponent};
use bevy_tweening::*;
use bevy_tweening::lens::SpriteColorLens;
use bevy_kira_audio::prelude::*;
use crate::GamePlugin::{Gameplay::Conductor::{self, ConductorEvents, SongHandle}, Core::SpriteXML::{SpriteXMLBundle3D, SpriteXML3D}};

#[derive(Resource, Default)]
pub struct FunkinAssets
{
    pub bumpin: Handle<Image>,
    pub yourgf: Handle<Image>,
    pub pressenter: Handle<Image>
}

#[derive(Component)]
pub struct Logo;

#[derive(Component)]
pub struct GF
{
    danceLeft: bool
}

pub fn title_init(asset_server: Res<AssetServer>, mut commands: Commands, mut sprite_params: Sprite3dParams, funkin: Res<FunkinAssets>)
{
    asset_server.mark_unused_assets();
    asset_server.free_unused_assets();
    let tween = Tween::new(EaseMethod::Linear, Duration::from_secs(2), SpriteColorLens
    {
        start: Color::rgba(1.0, 1.0, 1.0, 1.0),
        end: Color::rgba(1.0, 1.0, 1.0, 0.0)
    }).with_completed_event(0);

    commands.spawn((SpriteBundle
    {
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1280.0, 720.0, 2.0)),
        sprite: Sprite
        {
            color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            ..default()
        },
        ..default()
    }, Animator::new(tween)));
    
    let bf = sprite_params.atlases.add(TextureAtlas::new_empty(funkin.bumpin.clone(), Vec2::new(1850.0, 1400.0)));
    let gfs = sprite_params.atlases.add(TextureAtlas::new_empty(funkin.yourgf.clone(), Vec2::new(3660.0, 2670.0)));
    
    let mut bumpin = SpriteXMLBundle3D::new("assets/images/cool.xml".to_string(), &bf, &mut sprite_params, true).unwrap();
    bumpin.spritexml.add_anim_from_prefix("logo bumpin".to_string(), false, 24).unwrap();
    bumpin.sprite_sheet.pbr.transform.translation = Vec3::new(-2.0, 0.8, -6.0);
    bumpin.sprite_sheet.pbr.transform = bumpin.sprite_sheet.pbr.transform.with_scale(Vec3::new(0.7, 0.7, 0.7));
    bumpin.spritexml.apply_offsets(&bumpin.sprite_sheet.params, &mut bumpin.sprite_sheet.pbr.transform);
    bumpin.spritexml.set_anim("logo bump".to_string(), &mut bumpin.sprite_sheet.params, true).unwrap();
    
    // gf
    let mut gf = SpriteXMLBundle3D::new("assets/images/gfDanceTitle.xml".to_string(), &gfs, &mut sprite_params, true).unwrap();
    gf.spritexml.add_anim_from_indices("gfDance".to_string(), false, 24, &[30, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14], "danceLeft".to_string()).unwrap();
    gf.spritexml.add_anim_from_indices("gfDance".to_string(), false, 24, &[15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29], "danceRight".to_string()).unwrap();
    gf.sprite_sheet.pbr.transform.translation = Vec3::new(2.7, -0.3, -8.8);
    gf.spritexml.apply_offsets(&gf.sprite_sheet.params, &mut gf.sprite_sheet.pbr.transform);
    gf.spritexml.set_anim("danceLeft".to_string(), &mut gf.sprite_sheet.params, true).unwrap();
    commands.spawn((bumpin, Logo));
    commands.spawn((gf, GF {danceLeft: false}));
}

pub fn bye_flash(mut commands: Commands, mut reader: EventReader<TweenCompleted>)
{
    for ev in reader.iter()
    {
        if ev.user_data == 0
        {
            commands.entity(ev.entity).despawn();
        }
        
    }
}

pub fn beat_heat(mut reader: EventReader<ConductorEvents>, mut query: Query<&mut SpriteXML3D, With<Logo>>, mut querygf: Query<(&mut AtlasSprite3dComponent, &mut SpriteXML3D, &mut GF), (With<GF>, Without<Logo>)>)
{
    for ev in reader.iter()
    {
        match ev {
            ConductorEvents::BeatHit =>
            {
                for mut xml in query.iter_mut()
                {
                    xml.reset_anim_idx().unwrap();
                }
                for (mut sprite, mut xml, mut gf) in querygf.iter_mut()
                {
                    gf.danceLeft = !gf.danceLeft;
                    if gf.danceLeft
                    {
                        xml.set_anim("danceLeft".to_string(), &mut sprite, false).unwrap();
                        xml.reset_anim_idx().unwrap();
                    }
                    else {
                        
                        xml.set_anim("danceRight".to_string(), &mut sprite, false).unwrap();
                        xml.reset_anim_idx().unwrap();
                    }
                }
            },
            _ =>
            {
                
            }
        }
    }
}
pub fn handle_beatstate(mut conductor: ResMut<Conductor::Conductor>, writer: EventWriter<ConductorEvents>, handle: Res<SongHandle>, audio_instances: Res<Assets<AudioInstance>>, time: Res<Time>)
{
    conductor.update_beatstate(writer);
    if let Some(instance) = audio_instances.get(&handle.0)
    {
        match instance.state()
        {
            PlaybackState::Playing { position } =>
            {
                conductor.songPos = (position * 1000.0) + time.delta_seconds_f64();
            }
            _ =>
            {

            }
        }
    }
}