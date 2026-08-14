use log::{info, warn};
use std::error::Error;
use std::thread;
use std::time::Duration;

use crate::alsa;

struct Synth {
    fs: fluidlite::Synth,
    thread: std::thread::JoinHandle<()>,
}
impl Synth {
    pub fn try_init(sf2file: &str, banknum: i32) -> Result<Self, Box<dyn Error>> {
        let settings = fluidlite::Settings::new()?;
        let fs = fluidlite::Synth::new(settings)?;

        let thread = std::thread::spawn(move || {
            info!("Starting synth thread");
            loop {
                fs.process(64);
                thread::sleep(Duration::from_millis(1));
            }
        });

        Ok(Self { fs, thread })
    }
}

impl Drop for Synth {
    fn drop(&mut self) {
        info!("Shutting down synth");
        let _ = self.thread.join();
    }
}

pub fn beep(synth: &synth::Synth, note: i32, vol: i32) {
    const MIDI_CC_VOLUME: i32 = 7;
    synth.noteon(0, note, vol);
    synth.cc(0, MIDI_CC_VOLUME, vol);
    thread::sleep(Duration::from_millis(100));
    synth.noteoff(0, note);
}
