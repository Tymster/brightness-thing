use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::{mem, slice};

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

#[repr(C)]
pub struct InputEvent {
    tv_sec: u64,
    tv_usec: u64,
    evtype: u16,
    code: u16,
    value: i32,
}

impl InputEvent {
    pub fn from_reader(mut rdr: impl Read) -> Result<Self> {
        unsafe {
            let mut event = mem::zeroed();
            let buf = slice::from_raw_parts_mut(&mut event as *mut _ as _, mem::size_of::<Self>());
            rdr.read_exact(buf).unwrap();
            Ok(event)
        }
    }
}

fn main() -> Result {
    let input = File::open("/dev/input/event0").unwrap();
    loop {
        let ev = InputEvent::from_reader(&input)?;
        if ev.evtype == 1 && ev.value == 1 {
            let x = match ev.code {
                224 => -25,
                225 => 25,
                _ => continue,
            };
            let value: i32 = fs::read_to_string("/sys/class/backlight/apple-panel-bl/brightness")?
                .trim()
                .parse()?;
            let mut file = std::fs::File::create("/sys/class/backlight/apple-panel-bl/brightness")?;
            file.write_all((value + x).clamp(0, 400).to_string().as_bytes())?;
        }
    }
}
