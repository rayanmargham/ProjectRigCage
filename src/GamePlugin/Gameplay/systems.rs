use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::Song::{self, loadJson};

use super::Conductor;

use super::Conductor::SongHandle;

use super::PlayStateInformation;


#[derive(Resource)]
pub struct CountDownTimer(Timer, u8);


pub fn setup_state(
    mut commands: Commands,
    mut conductor: ResMut<Conductor::Conductor>,
    song_handle: Option<Res<SongHandle>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut playstate_information: Option<ResMut<PlayStateInformation>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>
)
{
    if let Some(hand) = song_handle
    {
        if let Some(instance) = audio_instances.get_mut(&hand.0)
        {
            instance.stop(AudioTween::default());
        }
    }
    commands.remove_resource::<SongHandle>(); // bye bozo
    if let None = playstate_information
    {
        panic!("Cannot Proceed without PlayState information, Consult the Doucmentation");
    }
    // we can continue 

    let song_name = &mut playstate_information.as_mut().unwrap().song;
    // i want to kill myself
    if song_name.as_str() == ""
    {
        song_name.push_str("index-hard");
    }
    let song_dir = format!("assets/playstate/{}/{}.json", song_name, song_name);
    let song_dir_actual = format!("playstate/{}", song_name);
    println!("{}", song_dir);

    let song = loadJson(&song_dir);
    match song
    {
        Some(song) => 
        {
            conductor.changeBPM(song.song.bpm);
            conductor.song = Some(song.song);
            conductor.mapBPMChanges();

            start_ze_down(commands, conductor, playstate_information.unwrap(), audio, asset_server);
        },
        None => panic!("Invaild Song!"),
    }
    

}

// gabe-in ga-ga-ga-ben g-a-a-gabin ga-ga-gain
// starts countdown
fn start_ze_down(
    mut commands: Commands,
    mut conductor: ResMut<Conductor::Conductor>,
    mut playstate_information: ResMut<PlayStateInformation>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>
)
{
    playstate_information.startedcountdown = true;
    conductor.songPos = -conductor.crochet * 5.0;

    commands.insert_resource(CountDownTimer(Timer::new(Duration::from_millis(conductor.crochet as u64), TimerMode::Once), 0));
}

pub fn countdown(
    mut commands: Commands,
    countdown: Option<ResMut<CountDownTimer>>,
    time: Res<Time>
)
{
    match countdown
    {
        Some(mut c) =>
        {
            // i wanna cut myself deeply in my skin and bleed out
            c.0.tick(time.delta());
            if c.0.just_finished()
            {
                match c.1
                {
                    0 =>
                    {
                        info!("3");
                    },
                    1 =>
                    {
                        info!("2");
                    },
                    2 =>
                    {
                        info!("1");
                    },
                    3 =>
                    {
                        info!("go");
                    },
                    
                    _ =>
                    {
                        
                    }
                }
                if c.1 != 5
                {
                    c.0.reset();
                    c.1 += 1;
                }
            }
            
        },
        None =>
        {
            
        }
    }
}