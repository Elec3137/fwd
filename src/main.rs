use std::{thread, time::Duration};

use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};

mod grid;
mod led;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    led::init()?;

    let mut sys = sysinfo::System::new();

    let mut grid0 = grid::Grid::default();
    let mut grid1 = grid::Grid::default();

    let mut strip0 = led::strip0()?;
    let mut strip1 = led::strip1()?;

    loop {
        sys.refresh_specifics(
            RefreshKind::nothing()
                .with_memory(MemoryRefreshKind::nothing().with_ram())
                .with_cpu(CpuRefreshKind::nothing().with_cpu_usage()),
        );
        grid0.clear();
        grid1.clear();

        for (x, cpu) in sys.cpus().iter().enumerate() {
            let cpu_usage = (cpu.cpu_usage() as f64 / (100. / led::HEIGHT as f64)).round() as usize;
            for y in 0..cpu_usage {
                if x < led::WIDTH - 1 {
                    grid0.set_xy(x, y, true);
                } else {
                    grid1.set_xy(x - led::WIDTH + 1, y, true);
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
            eprintln!("failed to write grid0: {e}")
        }
        if let Err(e) = grid1.write(&mut strip1) {
            eprintln!("failed to write grid1: {e}")
        }
        thread::sleep(Duration::from_millis(100));
    }

    // Ok(())
}
