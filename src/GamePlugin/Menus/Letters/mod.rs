use bevy::prelude::*;
use bevy_sprite3d::Sprite3dParams;

use crate::GamePlugin::Core::SpriteXML::{SpriteXMLBundle, SpriteXMLBundle3D};
use crate::GamePlugin::PreLoader::PreloadFunkinAssets;


#[derive(Bundle)]
pub struct LetterBundle
{
    sprite: SpriteXMLBundle,
    letter: Letter
}


#[derive(Bundle)]
pub struct LetterBundle3D
{
    sprite: SpriteXMLBundle3D,
    letter: Letter
}


#[derive(Component)]
pub struct Letter
{
    letter_anim: String
}

impl LetterBundle
{
    pub fn new(text: String, commands: &mut Commands, preloaded: &Res<PreloadFunkinAssets>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>, position: Transform) -> Option<Entity>
    {
        let bfs = texture_atlases.add(TextureAtlas::new_empty(preloaded.alphabet.clone(), Vec2::new(1024.0, 695.0)));
        let Some(bf) = texture_atlases.get_mut(&bfs) else { return None };
        let mut idx = 0;
        let mut entit: Entity = Entity::from_raw(0);
        let mut off = 50;
        for c in text.chars()
        {
            if idx == 0
            {
                entit = commands.spawn((SpatialBundle {transform: position, ..default()}, Name::new(text.clone()))).with_children(|pare|
                {
                    let mut xml_bundle = SpriteXMLBundle::new("assets/images/alphabet.xml".to_string(), &bfs, bf).unwrap();
                    let mut anim = c.to_lowercase().to_string();
                    anim.push_str(" bold");
                    xml_bundle.spritexml.add_anim_from_prefix(anim.clone(), true, 24).unwrap();
                    xml_bundle.spritexml.set_anim(anim, &mut xml_bundle.sprite_sheet.sprite, true).unwrap();
                    xml_bundle.spritexml.apply_offsets(&xml_bundle.sprite_sheet.sprite, &mut xml_bundle.sprite_sheet.transform);
                    
                    pare.spawn(xml_bundle);
                }).id();
                
                
            }
            else {
                commands.entity(entit).with_children(|par|
                {
                    if c != ' '
                    {
                        let mut xml_bundle = SpriteXMLBundle::new("assets/images/alphabet.xml".to_string(), &bfs, bf).unwrap();
                        let mut anim = c.to_lowercase().to_string();
                        anim.push_str(" bold");
                        xml_bundle.spritexml.add_anim_from_prefix(anim.clone(), true, 24).unwrap();
                        xml_bundle.spritexml.set_anim(anim, &mut xml_bundle.sprite_sheet.sprite, true).unwrap();
                        xml_bundle.spritexml.apply_offsets(&xml_bundle.sprite_sheet.sprite, &mut xml_bundle.sprite_sheet.transform);
                        
                        xml_bundle.sprite_sheet.transform.translation.x += off as f32;
                        off += 50;
                        par.spawn(xml_bundle);
                    }
                    else {
                        off += 50;
                    }
                    
                });
            }
            idx += 1; // used to keep track
        }
        return Some(entit);
    }
}

impl LetterBundle3D
{
    pub fn new(text: String, commands: &mut Commands, preloaded: &Res<PreloadFunkinAssets>, sprite_params: &mut Sprite3dParams, mut position: Transform, align_center: bool) -> Option<Entity>
    {
        
        let bfs = sprite_params.atlases.add(TextureAtlas::new_empty(preloaded.alphabet.clone(), Vec2::new(1024.0, 695.0)));
        let mut idx = 0;
        let mut entit: Entity = Entity::from_raw(0);
        let mut off = 0.5;
        if !align_center
        {
            for c in text.chars()
            {
                
                if idx == 0
                {
                    entit = commands.spawn((SpatialBundle {transform: position, ..default()}, Name::new(text.clone()))).with_children(|pare|
                    {
                        let mut xml_bundle = SpriteXMLBundle3D::new("assets/images/alphabet.xml".to_string(), &bfs, sprite_params, true).unwrap();
                        let mut anim = c.to_lowercase().to_string();
                        anim.push_str(" bold");
                        xml_bundle.spritexml.add_anim_from_prefix(anim.clone(), true, 24).unwrap();
                        xml_bundle.spritexml.set_anim(anim, &mut xml_bundle.sprite_sheet.params, true).unwrap();
                        xml_bundle.spritexml.apply_offsets(&xml_bundle.sprite_sheet.params, &mut xml_bundle.sprite_sheet.pbr.transform);
                        // xml_bundle.sprite_sheet.pbr.transform.translation += position.translation;
                        pare.spawn(xml_bundle);
                    }).id();
                    
                    
                }
                else {
                    commands.entity(entit).with_children(|par|
                    {
                        if c != ' '
                        {
                            let mut xml_bundle = SpriteXMLBundle3D::new("assets/images/alphabet.xml".to_string(), &bfs, sprite_params, true).unwrap();
                            let mut anim = c.to_lowercase().to_string();
                            anim.push_str(" bold");
                            xml_bundle.spritexml.add_anim_from_prefix(anim.clone(), true, 24).unwrap();
                            xml_bundle.spritexml.set_anim(anim, &mut xml_bundle.sprite_sheet.params, true).unwrap();
                            xml_bundle.spritexml.apply_offsets(&xml_bundle.sprite_sheet.params, &mut xml_bundle.sprite_sheet.pbr.transform);
                            // xml_bundle.sprite_sheet.pbr.transform.translation += position.translation;
                            xml_bundle.sprite_sheet.pbr.transform.translation.x += off;
                            off += 0.5;
                            par.spawn(xml_bundle);
                        }
                        else 
                        {
                            off += 0.5;
                        }
                        
                    });
                }
                idx += 1; // used to keep track
            }
        }
        else
        {
            let mut total_pos = 0.0;
            for c in text.chars()
            {
                //println!("{}", c);
                if idx == 0
                {
                    entit = commands.spawn((SpatialBundle {..default()}, Name::new(text.clone()))).with_children(|pare|
                    {
                        let mut xml_bundle = SpriteXMLBundle3D::new("assets/images/alphabet.xml".to_string(), &bfs, sprite_params, true).unwrap();
                        let mut anim = c.to_lowercase().to_string();
                        anim.push_str(" bold");
                        xml_bundle.spritexml.add_anim_from_prefix(anim.clone(), true, 24).unwrap();
                        xml_bundle.spritexml.set_anim(anim, &mut xml_bundle.sprite_sheet.params, true).unwrap();
                        xml_bundle.spritexml.apply_offsets(&xml_bundle.sprite_sheet.params, &mut xml_bundle.sprite_sheet.pbr.transform);
                        // xml_bundle.sprite_sheet.pbr.transform.translation += position.translation;
                        pare.spawn(xml_bundle);
                    }).id();
                    
                    
                }
                else {
                    commands.entity(entit).with_children(|par|
                    {
                        if c != ' '
                        {
                            let mut xml_bundle = SpriteXMLBundle3D::new("assets/images/alphabet.xml".to_string(), &bfs, sprite_params, true).unwrap();
                            let mut anim = c.to_lowercase().to_string();
                            anim.push_str(" bold");
                            xml_bundle.spritexml.add_anim_from_prefix(anim.clone(), true, 24).unwrap();
                            xml_bundle.spritexml.set_anim(anim, &mut xml_bundle.sprite_sheet.params, true).unwrap();
                            xml_bundle.spritexml.apply_offsets(&xml_bundle.sprite_sheet.params, &mut xml_bundle.sprite_sheet.pbr.transform);
                            // xml_bundle.sprite_sheet.pbr.transform.translation += position.translation;
                            xml_bundle.sprite_sheet.pbr.transform.translation.x += off;
                            off += 0.5;
                            // SHUT UP ILL HARD CODE THIS LTROLELDL
                            total_pos += match c {
                                'a' | 'c' =>
                                {
                                    0.55
                                },
                                'b' | 'n' | 'p' =>
                                {
                                    0.46
                                },
                                'd' =>
                                {
                                    0.53
                                },
                                'e' | 'f' | 'i' | 'k' | 't' | 'u' =>
                                {
                                    0.44
                                },
                                'g' | 'j' | 'v' | 'y'=>
                                {
                                    0.54
                                },
                                'h' =>
                                {
                                    0.45
                                },
                                'l' =>
                                {
                                    0.41
                                },
                                'm' | 'x'=>
                                {
                                    0.56
                                },
                                'o' =>
                                {
                                    0.50
                                },
                                'q' | 'z' =>
                                {
                                    0.52
                                },
                                'r' =>
                                {
                                    0.48
                                },
                                'w' =>
                                {
                                    0.59
                                },
                                _ =>
                                {
                                    0.5
                                }
        
                            };
                            par.spawn(xml_bundle);
                        }
                        else
                        {
                            off += 0.5;
                            total_pos += 0.5;
                        }
                        
                        
                    });
                }
                idx += 1; // used to keep track
            }
            position.translation.x += (-total_pos / 2.0) * position.scale.x;
            commands.entity(entit).insert(position);
        }
        return Some(entit);
    }
}