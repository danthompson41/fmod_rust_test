use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

use libfmod::{Error, LoadMemoryMode, SpeakerMode, StopMode, Studio};
use libfmod::ffi::{FMOD_INIT_NORMAL, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_LOAD_BANK_NORMAL};

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    studio.initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;
    let master = studio.load_bank_file("./Master.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    let strings = studio.load_bank_file("./Master.strings.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    let step_event = studio.get_event("event:/step")?;
    let step = step_event.create_instance();
    let step2 = step_event.create_instance();
    step?.start();
    studio.update();
    sleep(Duration::from_secs(1));
    step2?.start();
    studio.update();
    sleep(Duration::from_secs(1));
    dbg!(master.get_event_list(5));
    Ok(())
}
