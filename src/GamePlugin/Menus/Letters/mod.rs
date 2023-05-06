use bevy::prelude::*;

use crate::GamePlugin::Core::SpriteXML::SpriteXMLBundle;
use crate::GamePlugin::PreLoader::PreloadFunkinAssets;


#[derive(Bundle)]
pub struct LetterBundle
{
    sprite: SpriteXMLBundle,
    letter: Letter
}

#[derive(Component)]
pub struct Letter
{
    letter_anim: String
}

impl LetterBundle
{
    pub fn new(text: String, commands: &mut Commands, preloaded: &Res<PreloadFunkinAssets>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>, position: Transform)
    {
        let bfs = texture_atlases.add(TextureAtlas::new_empty(preloaded.alphabet.clone(), Vec2::new(1024.0, 695.0)));
        let Some(bf) = texture_atlases.get_mut(&bfs) else { return };
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
                    xml_bundle.sprite_sheet.transform.translation += position.translation;
                    pare.spawn(xml_bundle);
                }).id();
                
                
            }
            else {
                commands.entity(entit).with_children(|par|
                {
                    let mut xml_bundle = SpriteXMLBundle::new("assets/images/alphabet.xml".to_string(), &bfs, bf).unwrap();
                    let mut anim = c.to_lowercase().to_string();
                    anim.push_str(" bold");
                    xml_bundle.spritexml.add_anim_from_prefix(anim.clone(), true, 24).unwrap();
                    xml_bundle.spritexml.set_anim(anim, &mut xml_bundle.sprite_sheet.sprite, true).unwrap();
                    xml_bundle.spritexml.apply_offsets(&xml_bundle.sprite_sheet.sprite, &mut xml_bundle.sprite_sheet.transform);
                    xml_bundle.sprite_sheet.transform.translation += position.translation;
                    xml_bundle.sprite_sheet.transform.translation.x += off as f32;
                    off += 50;
                    par.spawn(xml_bundle);
                });
            }
            idx += 1; // used to keep track
        }
    }
}