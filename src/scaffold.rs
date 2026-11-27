use std::{io, thread, time::Duration};

use crate::{grid, led};

pub struct State {
    strip0: Box<dyn serialport::SerialPort>,
    strip1: Box<dyn serialport::SerialPort>,
    pub grid0: grid::Grid,
    pub grid1: grid::Grid,
}

impl State {
    pub fn new() -> serialport::Result<Self> {
        let (strip0, strip1) = led::get_strip_pair()?;
        thread::sleep(led::MIN_INTERVAL);
        Ok(Self {
            strip0,
            strip1,
            grid0: Default::default(),
            grid1: Default::default(),
        })
    }
    pub fn run(&mut self, mut f: impl FnMut(&mut Self)) -> ! {
        loop {
            f(self);

            if let Err(e) = self.grid0.write(&mut self.strip0) {
                match e.kind() {
                    io::ErrorKind::BrokenPipe => match led::get_strip_pair() {
                        Ok(pair) => (self.strip0, self.strip1) = pair,
                        Err(e) => {
                            eprintln!("failed to reconnect strips: {e}");
                            thread::sleep(Duration::from_secs(1));
                        }
                    },
                    _ => {
                        eprintln!("unhandled write error: {e}");
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }
            if let Err(e) = self.grid1.write(&mut self.strip1) {
                match e.kind() {
                    io::ErrorKind::BrokenPipe => match led::get_strip_pair() {
                        Ok(pair) => (self.strip0, self.strip1) = pair,
                        Err(e) => {
                            eprintln!("failed to reconnect strips: {e}");
                            thread::sleep(Duration::from_secs(1));
                        }
                    },
                    _ => {
                        eprintln!("unhandled write error: {e}");
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }

            thread::sleep(led::MIN_INTERVAL);
        }
    }
}
