use std::{thread, time::Duration};

use serialport::{SerialPort, SerialPortType, UsbPortInfo};

/// https://github.com/FrameworkComputer/inputmodule-rs/blob/main/commands.md
pub const COMMAND_BYTES: [u8; COMMAND_LEN] = [50, 172, 6];
pub const COMMAND_LEN: usize = 3;

pub const WIDTH: usize = 9;
pub const HEIGHT: usize = 34;

const MAX_BRIGHTNESS: &[u8] = &[50, 172, 0, 255];
const CLEAR: &[u8] = &[50, 172, 8];

/// about 17 milliseconds;
/// the amount of time that the program should wait between sending commands.
///
/// determined experimentally, any shorter than this (accounting for overhead)
/// would cause "Operation timed out" errors upon writing.
///
/// note that the nanosecond precision is a guess;
/// we found errors at 16ms and no errors at 17ms,
/// and with that information decided 60fps is likely what it's made for.
pub const MIN_INTERVAL: Duration = Duration::from_nanos(16_666_666);

pub fn init() -> serialport::Result<()> {
    let (mut strip0, mut strip1) = get_strip_pair()?;

    strip0.write_all(MAX_BRIGHTNESS)?;
    strip1.write_all(MAX_BRIGHTNESS)?;

    // Sleep to prevent errors
    thread::sleep(MIN_INTERVAL);

    strip0.write_all(CLEAR)?;
    strip1.write_all(CLEAR)?;

    Ok(())
}

pub fn get_strip_pair() -> serialport::Result<(Box<dyn SerialPort>, Box<dyn SerialPort>)> {
    let mut strips = get_strips()?;
    Ok((
        strips.next().ok_or(serialport::Error::new(
            serialport::ErrorKind::NoDevice,
            "missing 'LED Matrix Input Module'",
        ))??,
        strips.next().ok_or(serialport::Error::new(
            serialport::ErrorKind::NoDevice,
            "missing 'LED Matrix Input Module'",
        ))??,
    ))
}

pub fn get_strips()
-> serialport::Result<impl Iterator<Item = serialport::Result<Box<dyn SerialPort>>>> {
    Ok(serialport::available_ports()?
        .into_iter()
        .filter(|info| {
            matches!(
                &info.port_type,
                SerialPortType::UsbPort(UsbPortInfo {
                    product: Some(product),
                    ..
                }) if product == "LED Matrix Input Module"
            )
        })
        .map(|info| serialport::new(&info.port_name, 115200).open()))
}
