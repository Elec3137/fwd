use std::{io, thread, time::Duration};

use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};

mod grid;
mod led;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let specifics = RefreshKind::nothing()
        .with_memory(MemoryRefreshKind::nothing().with_ram())
        .with_cpu(CpuRefreshKind::nothing().with_usage());
    let mut sys = sysinfo::System::new_with_specifics(specifics)?;

    let mut grid0 = grid::Grid::default();
    let mut grid1 = grid::Grid::default();

    let (mut strip0, mut strip1) = led::get_strip_pair()?;
    thread::sleep(led::MIN_INTERVAL);
    loop {
        sys.refresh_specifics(specifics);
        grid0.clear();
        grid1.clear();

        for (x, cpu) in sys.cpus().iter().enumerate() {
            let cpu_usage = (cpu.usage() as f64 / (100. / led::HEIGHT as f64)).round() as usize;
            for y in 0..cpu_usage {
                if x < led::WIDTH {
                    grid0.set_xy(x, y, true);
                } else {
                    grid1.set_xy(x - led::WIDTH, y, true);
                }
            }
        }

        let ram_usage = ((sys.used_memory() as f64 / sys.total_memory() as f64)
            * led::HEIGHT as f64)
            .round() as usize;
        for y in 0..ram_usage {
            grid1.set_xy(led::WIDTH - 1, y, true);
        }

        if let Err(e) = grid0.write(&mut strip0) {
            match e.kind() {
                io::ErrorKind::BrokenPipe => match led::get_strip_pair() {
                    Ok(pair) => (strip0, strip1) = pair,
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
        if let Err(e) = grid1.write(&mut strip1) {
            match e.kind() {
                io::ErrorKind::BrokenPipe => match led::get_strip_pair() {
                    Ok(pair) => (strip0, strip1) = pair,
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

    // Ok(())
}
