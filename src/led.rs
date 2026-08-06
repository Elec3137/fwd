use std::{thread, time::Duration};

use serialport::SerialPort;

/// https://github.com/FrameworkComputer/inputmodule-rs/blob/main/commands.md
pub const COMMAND_BYTES: [u8; COMMAND_LEN] = [50, 172, 6];
pub const COMMAND_LEN: usize = 3;

pub const WIDTH: usize = 9;
pub const HEIGHT: usize = 34;

const MAX_BRIGHTNESS: &[u8] = &[50, 172, 0, 255];
const CLEAR: &[u8] = &[50, 172, 8];

pub fn init() -> serialport::Result<()> {
    let mut strip0 = strip0()?;
    let mut strip1 = strip1()?;

    strip0.write_all(MAX_BRIGHTNESS)?;
    strip1.write_all(MAX_BRIGHTNESS)?;

    // Sleep to prevent errors
    thread::sleep(Duration::from_secs(1));

    strip0.write_all(CLEAR)?;
    strip1.write_all(CLEAR)?;

    Ok(())
}

pub fn strip0() -> serialport::Result<Box<dyn SerialPort>> {
    serialport::new("/dev/ttyACM0", 115200).open()
}
pub fn strip1() -> serialport::Result<Box<dyn SerialPort>> {
    serialport::new("/dev/ttyACM1", 115200).open()
}
