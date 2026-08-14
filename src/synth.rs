use log::{info, warn};
use std::error::Error;
use std::thread;
use std::time::Duration;

use crate::alsa;

pub struct Synth {
    fs: fluidlite::Synth,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Synth {
    pub fn try_init(sf2file: &str, banknum: i32) -> Result<Self, Box<dyn Error>> {
        let settings = fluidlite::Settings::new()?;
        let fs = fluidlite::Synth::new(settings)?;
        fs.sfload("/usr/share/sounds/sf2/TimGM6mb.sf2", true)?;

        let thread = std::thread::spawn(move || {
            info!("Starting synth thread");
        });

        Ok(Self {
            fs,
            thread: Some(thread),
        })
    }

    pub fn noteon(&self, channel: u8, note: i32, volume: i32) {}

    pub fn noteoff(&self, channel: u8, note: i32) {}

    pub fn program_change(&self, channel: u8, program: i32) {}

    pub fn cc(&self, channel: u8, control: i32, value: i32) {}
}

impl Drop for Synth {
    fn drop(&mut self) {
        info!("Shutting down synth");
        match self.thread.take() {
            Some(handle) => {
                let _ = handle.join();
            }
            None => {}
        }
    }
}

pub fn beep(synth: &Synth, note: i32, vol: i32) {
    const MIDI_CC_VOLUME: i32 = 7;
    synth.noteon(0, note, vol);
    synth.cc(0, MIDI_CC_VOLUME, vol);
    thread::sleep(Duration::from_millis(100));
    synth.noteoff(0, note);
}
