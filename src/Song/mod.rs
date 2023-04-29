use serde::{Serialize, Deserialize};
use std::fs;
#[derive(Serialize, Deserialize, Debug)]
pub struct sectionNote
{
    posMS: f32,
    strum: i32, // Arrow Type
    lengthNote: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwaggersSection
{
    pub sectionNotes: Vec<sectionNote>,
    #[serde(default = "default_beats")]
    pub sectionBeats: f32,
    #[serde(default = "default_section")]
    pub typeOfSection: i32,
    pub mustHitSection: bool,
    #[serde(default = "default_bpm")]
    pub bpm: f32,
    #[serde(default = "default_changebpm")]
    pub changeBPM: bool,
    #[serde(default = "default_altanim")]
    pub altAnim: bool,
    #[serde(default = "default_gf")]
    pub gfSection: bool
}

fn default_gf() -> bool
{
    false
}
fn default_altanim() -> bool
{
    false
}
fn default_changebpm() -> bool
{
    false
}
fn default_section() -> i32
{
    0
}
fn default_bpm() -> f32
{
    -1.0
}
fn default_beats() -> f32
{
    4.0
}
fn default_valid() -> bool
{
    true
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SwagSong
{
    // Oi mate you wanna go to the pub LMAOO
    // don't ask why i did this shit i just don't wanna make setters and getts and do that OO shit
    // java more like my ass LOOL
    pub song: String,
    pub notes: Vec<SwaggersSection>,
    pub bpm: f32,
    pub needsVoices: bool,
    pub speed: f32,

    pub player1: String,
    pub player2: String,
    #[serde(default = "default_valid")]
    pub validScore: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Song
{
    pub song: SwagSong
}

pub fn loadJson(path: &str) -> Option<Song>
{
    let file = fs::read_to_string(path);
    match file
    {
        Ok(content) =>
        {
            let mut swag: Song = serde_json::from_str(&content).unwrap();
            swag.song.validScore = true;
            return Some(swag);
        }
        Err(e) =>
        {
            println!("{}: Couldn't Read Json!", file!());
            println!("{}: Error: {}", file!(), e);
            None
        }
    }
}