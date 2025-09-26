use anyhow::Context;
use aya::maps::HashMap;
use aya::programs::KProbe;
use aya::{include_bytes_aligned, Ebpf};
use aya_log::EbpfLogger;
use log::{info, warn};
use tokio::signal;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    #[cfg(debug_assertions)]
    let mut bpf = Ebpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/kernel-space"
    ))?;

    #[cfg(not(debug_assertions))]
    let mut bpf = Ebpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/kernel-space"
    ))?;

    if let Err(e) = EbpfLogger::init(&mut bpf) {
        warn!("failed to initialize eBPF logger: {}", e);
    }

    // Attach to the 'read' syscall
    let read_prog: &mut KProbe = bpf.program_mut("read_counter").unwrap().try_into()?;
    read_prog.load()?;
    read_prog.attach("__x64_sys_read", 0)?;
    info!("Attached kprobe to __x64_sys_read");

    // Attach to the 'write' syscall
    let write_prog: &mut KProbe = bpf.program_mut("write_counter").unwrap().try_into()?;
    write_prog.load()?;
    write_prog.attach("__x64_sys_write", 0)?;
    info!("Attached kprobe to __x64_sys_write");

    // Attach to the 'open' syscall
    let open_prog: &mut KProbe = bpf.program_mut("open_counter").unwrap().try_into()?;
    open_prog.load()?;
    open_prog.attach("__x64_sys_open", 0)?;
    info!("Attached kprobe to __x64_sys_open");

    let counts_map_generic = bpf
        .map_mut("SYSCALL_COUNTS")
        .context("Failed to find the SYSCALL_COUNTS map")?;
    let counts_map: HashMap<_, u32, u64> = HashMap::try_from(counts_map_generic)?;

    info!("Waiting for Ctrl-C to exit...");

    let mut interval = time::interval(Duration::from_secs(2));
    loop {
        tokio::select! {
            _ = interval.tick() => {
                let read_count = counts_map.get(&0, 0).unwrap_or(0);
                let write_count = counts_map.get(&1, 0).unwrap_or(0);
                let open_count = counts_map.get(&2, 0).unwrap_or(0);

                println!("---------------------------------");
                println!("Read calls:  {}", read_count);
                println!("Write calls: {}", write_count);
                println!("Open calls:  {}", open_count);
            }
            _ = signal::ctrl_c() => {
                info!("Exiting...");
                break;
            }
        }
    }

    Ok(())
}
