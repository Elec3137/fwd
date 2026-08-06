use std::{thread, time::Duration};

use serialport::SerialPort;

/// https://github.com/FrameworkComputer/inputmodule-rs/blob/main/commands.md
pub const COMMAND_BYTES: [u8; COMMAND_LEN] = [50, 172, 6];
pub const COMMAND_LEN: usize = 3;

pub const WIDTH: usize = 9;
pub const HEIGHT: usize = 34;

pub fn init() -> serialport::Result<()> {
    let mut strip0 = strip0()?;
    let mut strip1 = strip1()?;

    // Set brightness
    strip0.write_all(&[50, 172, 0, 255])?;
    strip1.write_all(&[50, 172, 0, 255])?;

    // Sleep to prevent errors
    thread::sleep(Duration::from_secs(1));

    // Clear the LEDs
    strip0.write_all(&[50, 172, 8])?;
    strip1.write_all(&[50, 172, 8])?;

    Ok(())
}

pub fn strip0() -> serialport::Result<Box<dyn SerialPort>> {
    serialport::new("/dev/ttyACM0", 115200).open()
}
pub fn strip1() -> serialport::Result<Box<dyn SerialPort>> {
    serialport::new("/dev/ttyACM1", 115200).open()
}
