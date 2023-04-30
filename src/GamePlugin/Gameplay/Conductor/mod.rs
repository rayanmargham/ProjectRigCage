use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::Song::SwagSong;

pub struct BPMChangeEvent 
{
    stepTime: i32,            // time in steps (16th notes)
    songTime: f32,            // time in songTime (explained further below)
    bpm: f32,                 // bpm to change to
    stepCrochet: Option<f32>, // optional variable for the stepCrochet
}
#[derive(Resource)]
pub struct Conductor 
{
    bpm: f32,         // beats per minute
    crochet: f32,     // beats in milliseconds
    stepCrochet: f32, // "steps" (16th notes in music notation) in MS
    pub songPos: f32,     // position of the song in MS
    // lastSongPos is completely useless lmaoooo the game's code doesn't even utilize
    // it completely also btw this nvchad line editing thing sucks lmaoooo
    offset: f32, // used to offset the song position if you ever wanted to use this lolll
    safeZoneOffset: f32, // TODO: Comment this lmao
    bpmChangeMap: Vec<BPMChangeEvent>, // vec of BPMChangeEvent's
    curSection: i32,
    stepsToDo: i32,

    curStep: i32,
    curBeat: i32,

    curDecStep: f32,
    curDecBeat: f32,

    pub song: Option<SwagSong>
}
#[derive(Debug)]
pub enum ConductorEvents
{
    BeatHit,
    SectionHit,
    StepHit,
}
pub struct ConductorEvent(pub ConductorEvents);
#[derive(Resource)]
pub struct SongHandle(pub Handle<AudioInstance>);


impl FromWorld for Conductor
{
    fn from_world(world: &mut World) -> Self 
    {
        println!("{}: Init!", file!());
        Conductor::new()
    }
}
impl Conductor 
{
    fn new() -> Conductor
    {
        Conductor
        {
            bpm: 0.0,
            crochet: 0.0,
            stepCrochet: 0.0,
            songPos: 0.0,

            offset: 0.0,
            safeZoneOffset: (10.0 / 60.0) * 1000.0,
            bpmChangeMap: Vec::new(),
            curSection: 0,
            stepsToDo: 0,

            curStep: 0,
            curBeat: 0,

            curDecBeat: 0.0,
            curDecStep: 0.0,

            song: None
        }
    }
    fn getBPMFromStep(&self, step: i32) -> BPMChangeEvent
    {
        let mut lastChange: BPMChangeEvent = BPMChangeEvent
        {
            stepTime: 0,
            songTime: 0.0,
            bpm: self.bpm,
            stepCrochet: Some(self.stepCrochet)
        };
        for i in 0..self.bpmChangeMap.len()
        {
            let MapInQuestion: &BPMChangeEvent = &self.bpmChangeMap[i];
            if (MapInQuestion.stepTime <= step)
            {
                lastChange.stepTime = MapInQuestion.stepTime;
                lastChange.songTime = MapInQuestion.songTime;
                lastChange.bpm = MapInQuestion.bpm;
                lastChange.stepCrochet = MapInQuestion.stepCrochet;
            }
        }
        return lastChange;
    }
    fn getBPMFromSeconds(&self, time: f32) -> BPMChangeEvent
    {
        let mut lastChange = BPMChangeEvent 
        {
            stepTime: 0,
            songTime: 0.0,
            bpm: self.bpm,
            stepCrochet: Some(self.stepCrochet),
        };
        for i in 0..self.bpmChangeMap.len()
        {
            let MapInQuestion: &BPMChangeEvent = &self.bpmChangeMap[i];
            if time >= MapInQuestion.songTime
            {
                lastChange.stepTime = MapInQuestion.stepTime;
                lastChange.songTime = MapInQuestion.songTime;
                lastChange.bpm = MapInQuestion.bpm;
                lastChange.stepCrochet = MapInQuestion.stepCrochet;
            }
        }
        return lastChange;
    }

    fn beatToSeconds(&self, beat: f32) -> f32
    {
        let step = (beat * 4.0) as i32;
        let lastChange = self.getBPMFromStep(step);
        return lastChange.songTime + (((step - lastChange.stepTime) as f32) / (lastChange.bpm / 60.0)/4.0) * 1000.0;
    }

    fn getStep(&self, time: f32) -> f32
    {
        let lastChange = self.getBPMFromSeconds(time);
        match lastChange.stepCrochet
        {
            Some(t) =>
            {
                return lastChange.songTime + (time - lastChange.songTime) / t;
            }
            None =>
            {
                return lastChange.songTime + (time - lastChange.songTime) / self.stepCrochet;
            }
        }
    }
    fn getStepRounded(&self, time: f32) -> f32
    {
        let lastChange = self.getBPMFromSeconds(time);
        match lastChange.stepCrochet
        {
            Some(t) =>
            {
                return lastChange.songTime + ((time - lastChange.songTime) / t).floor();
            }
            None =>
            {
                return lastChange.songTime + ((time - lastChange.songTime) / self.stepCrochet).floor();
            }
        }
    }

    fn getBeat(&self, time: f32) -> i32
    {
        return (self.getStep(time) as i32) / 4;
    }
    fn getBeatRounded(&self, time: f32) -> i32
    {
        return (self.getStepRounded(time).floor() as i32) / 4;
    }
    fn calculateCrochet(bpm: f32) -> f32
    {
        return (60.0/bpm)*1000.0
    }
    fn getSectionBeats(&self, section: i32) -> i32
    {
        if section < (self.song.as_ref().unwrap().notes.len() as i32)
        {
            return self.song.as_ref().unwrap().notes[section as usize].sectionBeats.floor() as i32;
        }
        else 
        {
            return 4;
        }
    }
    pub fn mapBPMChanges(&mut self)
    {
        let mut curBPM: f32 = self.song.as_ref().unwrap().bpm;
        let mut totalSteps: i32 = 0;
        let mut totalPos: f32 = 0.0;

        for i in 0..self.song.as_ref().unwrap().notes.len()
        {
            if self.song.as_ref().unwrap().notes[i].changeBPM && self.song.as_ref().unwrap().notes[i].bpm != curBPM
            {
                curBPM = self.song.as_ref().unwrap().notes[i].bpm;
                let event: BPMChangeEvent = BPMChangeEvent { stepTime: totalSteps, songTime: totalPos, bpm: curBPM, stepCrochet: Some(Conductor::calculateCrochet(curBPM)/4.0) };
                self.bpmChangeMap.push(event);
            }
            let deltaSteps: i32 = self.getSectionBeats(i as i32) * 4;
            totalSteps += deltaSteps;
            totalPos += ((60.0 / curBPM) * 1000.0 / 4.0) * (deltaSteps as f32);
            println!("{}: Mapping BPM...", file!());

        }
        println!("{}: New BPM Map BUDDY!", file!());
    }
    
    pub fn changeBPM(&mut self, newBpm: f32)
    {
        self.bpm = newBpm;
        
        self.crochet = Conductor::calculateCrochet(self.bpm);
        self.stepCrochet = self.crochet / 4.0;
    }

    pub fn update_beatstate(&mut self, mut writer: EventWriter<ConductorEvent>)
    {
        let oldStep: i32 = self.curStep;

        self.updateCurStep();
        self.updateBeat();

        if oldStep != self.curStep
        {
            if self.curStep > 0
            {
                // step hit
                if self.curStep % 4 == 0
                {
                    println!("{}: BEAT HIT AT {} LOOOL", file!(), self.curBeat);
                    writer.send(ConductorEvent(ConductorEvents::BeatHit));
                }
            }
            

            if oldStep < self.curStep
            {
                self.updateSection()
            }
            else 
            {
                self.rollbackSection()
            }
        }
    }
    
    fn updateCurStep(&mut self)
    {
        let lastChange = self.getBPMFromSeconds(self.songPos);

        let shit = ((self.songPos /* - Prefs.noteOffset*/) - lastChange.songTime) / lastChange.stepCrochet.unwrap();
        self.curDecStep = lastChange.stepTime as f32 + shit;
        self.curStep = lastChange.stepTime + shit.floor() as i32;
    }

    fn updateBeat(&mut self)
    {
        self.curBeat = self.curStep / 4;
        self.curDecBeat = self.curDecStep / 4.0;
    }

    fn getBeatsOnSection(&self) -> f32
    {
        if (self.curSection as usize) < self.song.as_ref().unwrap().notes.len()
        {
            return self.song.as_ref().unwrap().notes[self.curSection as usize].sectionBeats;
        }
        else
        {
            return 4.0;
        }
    }

    fn updateSection(&mut self)
    {
        if self.stepsToDo < 1 {self.stepsToDo = (self.getBeatsOnSection() * 4.0).round() as i32}
        while self.curStep >= self.stepsToDo
        {
            self.curSection += 1;
            let beats: f32 = self.getBeatsOnSection();
            self.stepsToDo += (beats * 4.0).round() as i32;
            // section hit
            if self.song.as_ref().unwrap().notes[self.curSection as usize].changeBPM
            {
                self.changeBPM(self.song.as_ref().unwrap().notes[self.curSection as usize].bpm);
                println!("{}: Changing BPM!", file!());
            }
        }
    }
    fn rollbackSection(&mut self)
    {
        if self.curStep < 0 {return};
        let lastSection: i32 = self.curSection;
        self.curSection = 0;
        self.stepsToDo = 0;

        for i in 0..self.song.as_ref().unwrap().notes.len()
        {
            if i < self.song.as_ref().unwrap().notes.len()
            {
                self.stepsToDo += (self.getBeatsOnSection() * 4.0).round() as i32;
                if (self.stepsToDo > self.curStep) {break}
                self.curSection += 1;
                
            }
        }

    }

}
