use alsa::{
    pcm::{Access, Format, HwParams, PCM},
    Direction, ValueOr,
};
use log::{debug, info, warn};
use std::error::Error;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::{array::TryFromSliceError, convert::TryInto};
use thread_priority::{
    set_thread_priority_and_policy, thread_native_id, RealtimeThreadSchedulePolicy, ThreadPriority,
    ThreadSchedulePolicy,
};

#[derive(Debug)]
enum Event {
    Noteon { note: i32, volume: i32 },
    Noteoff { note: i32 },
    ProgramChange { program: i32 },
    CC { control: i32, value: i32 },
}

pub struct Synth {
    tx: mpsc::Sender<Event>,
    thread: Option<std::thread::JoinHandle<()>>,
}

const SAMPLE_RATE: u32 = 48000;
const CHANNELS: u32 = 2;
const PERIODS: u32 = 3;
const PERIOD_SIZE: usize = 64;

fn setup_synth(soundfont: &str, prog: i32) -> Result<fluidlite::Synth, Box<dyn Error>> {
    let settings = fluidlite::Settings::new()?;
    let fs = fluidlite::Synth::new(settings)?;
    fs.sfload(soundfont, true)?;
    fs.program_change(0, prog as u32)?;
    Ok(fs)
}

fn run_synth(rx: mpsc::Receiver<Event>, fs: fluidlite::Synth) -> Result<(), Box<dyn Error>> {
    info!("Starting synth thread");

    match set_thread_priority_and_policy(
        thread_native_id(),
        ThreadPriority::Crossplatform(99u8.try_into().unwrap()),
        ThreadSchedulePolicy::Realtime(RealtimeThreadSchedulePolicy::Fifo),
    ) {
        Err(e) => {
            warn!("Setting realtime thread priority failed: {:?}", e);
        }
        Ok(_) => todo!(),
    }

    let pcm = PCM::new("default", Direction::Playback, false)?;
    {
        let hwp = HwParams::any(&pcm)?;
        hwp.set_channels(CHANNELS)?;
        hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest)?;
        hwp.set_format(Format::s16())?; // interleaved 16-bit signed PCM
        hwp.set_access(Access::RWInterleaved)?;
        hwp.set_periods(PERIODS, ValueOr::Nearest)?;
        hwp.set_period_size(PERIOD_SIZE as i64, ValueOr::Nearest)?;
        pcm.hw_params(&hwp)?;
    }
    let io = pcm.io_i16()?;

    let mut buffer = vec![0i16; PERIOD_SIZE * CHANNELS as usize];

    'outer: loop {
        let mut events = 0;
        loop {
            match rx.try_recv() {
                Ok(Event::Noteon { note, volume }) => {
                    events += 1;
                    //info!("Got noteon {note} {volume}");
                    fs.note_on(0, note.try_into().unwrap(), volume.try_into().unwrap())?;
                }
                Ok(Event::Noteoff { note }) => {
                    events += 1;
                    //info!("Got noteoff {note}");
                    fs.note_off(0, note.try_into().unwrap())?;
                }
                Ok(Event::ProgramChange { program }) => {
                    events += 1;
                    fs.program_change(0, program.try_into().unwrap())?;
                }
                Ok(Event::CC { control, value }) => {
                    events += 1;
                    fs.cc(0, control.try_into().unwrap(), value.try_into().unwrap())?;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break 'outer,
            }
        }
        if events > 0 {
            debug!("handled {events} events");
        }

        fs.write(&mut buffer[..])?;
        io.writei(&buffer)?;
    }

    pcm.drain()?;
    Ok(())
}

impl Synth {
    pub fn try_init(sf2file: &str, banknum: i32) -> Result<Self, Box<dyn Error>> {
        let fs = setup_synth(sf2file, banknum)?;
        let (tx, rx) = mpsc::channel::<Event>();
        let thread = std::thread::spawn(move || {
            run_synth(rx, fs).expect("failed to run synth");
        });
        Ok(Self {
            tx,
            thread: Some(thread),
        })
    }

    pub fn noteon(&self, _channel: u8, note: i32, volume: i32) {
        self.tx
            .send(Event::Noteon { note, volume })
            .expect("failed to send noteon message");
    }

    pub fn noteoff(&self, _channel: u8, note: i32) {
        self.tx
            .send(Event::Noteoff { note })
            .expect("failed to send noteoff message");
    }

    pub fn program_change(&self, _channel: u8, program: i32) {
        self.tx
            .send(Event::ProgramChange { program })
            .expect("failed to send program_change message");
    }

    pub fn cc(&self, _channel: u8, control: i32, value: i32) {
        self.tx
            .send(Event::CC { control, value })
            .expect("failed to send cc message");
    }
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
