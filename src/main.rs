use fwd::{led, scaffold};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let specifics = RefreshKind::nothing()
        .with_memory(MemoryRefreshKind::nothing().with_ram())
        .with_cpu(CpuRefreshKind::nothing().with_usage());
    let mut sys = sysinfo::System::new_with_specifics(specifics)?;

    let mut scaffold = scaffold::State::new()?;
    scaffold.run(|state| {
        sys.refresh_specifics(specifics);
        state.grid0.clear();
        state.grid1.clear();

        for (x, cpu) in sys.cpus().iter().enumerate() {
            let cpu_usage = (cpu.usage() as f64 / (100. / led::HEIGHT as f64)).round() as usize;
            for y in 0..cpu_usage {
                if x < led::WIDTH {
                    state.grid0.set_xy(x, y, true);
                } else {
                    state.grid1.set_xy(x - led::WIDTH, y, true);
                }
            }
        }

        let ram_usage = ((sys.used_memory() as f64 / sys.total_memory() as f64)
            * led::HEIGHT as f64)
            .round() as usize;
        for y in 0..ram_usage {
            state.grid1.set_xy(led::WIDTH - 1, y, true);
        }
    })

    // Ok(())
}
