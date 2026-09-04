use sysinfo::{Disks, System};

use crate::utils::command::create_command;

pub fn update_system() -> Result<(), String> {
    let mut cmd = create_command("sudo", ["pacman", "-Syu"]);
    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    child.wait().map_err(|e| e.to_string())?;

    Ok(())
}

pub fn shutdown() -> Result<(), String> {
    let mut cmd = create_command("sudo", ["shutdown", "now"]);
    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    child.wait().map_err(|e| e.to_string())?;

    Ok(())
}

pub fn restart() -> Result<(), String> {
    let mut cmd = create_command("sudo", ["reboot"]);
    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    child.wait().map_err(|e| e.to_string())?;

    Ok(())
}

pub fn system_status() -> Result<(), String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let os_name = System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown Version".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown Kernel".to_string());
    let host_name = System::host_name().unwrap_or_else(|| "Unknown Host".to_string());
    let uptime = System::uptime();
    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;

    println!("Host: {}", host_name);
    println!("OS: {} ({})", os_name, os_version);
    println!("Kernel: {}", kernel_version);
    println!("Uptime: {}h {}m {}s", hours, minutes, seconds);
    println!("------------------------------------------");

    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let free_mem = sys.free_memory();
    let mem_percent = if total_mem > 0 {
        used_mem as f64 / total_mem as f64 * 100.0
    } else {
        0.0
    };

    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let free_swap = sys.free_swap();
    let swap_percent = if total_swap > 0 {
        used_swap as f64 / total_swap as f64 * 100.0
    } else {
        0.0
    };

    fn to_gb(bytes: u64) -> f64 {
        bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    println!("MEMORY (RAM):");
    println!(
        "   |- Used: {:.2} GB / {:.2} GB ({:.1}%)",
        to_gb(used_mem),
        to_gb(total_mem),
        mem_percent
    );
    println!("   |- Free: {:.2} GB", to_gb(free_mem));

    println!("SWAP:");
    println!(
        "   |- Used: {:.2} GB / {:.2} GB ({:.1}%)",
        to_gb(used_swap),
        to_gb(total_swap),
        swap_percent
    );
    println!("   |- Free: {:.2} GB", to_gb(free_swap));
    println!("------------------------------------------");

    let cpus = sys.cpus();
    let global_cpu_usage = sys.global_cpu_usage();
    if let Some(first_cpu) = cpus.first() {
        println!("CPU Model: {}", first_cpu.brand());
        println!("   |- Frequency: {} MHz", first_cpu.frequency());
        println!("   |- Total Cores: {}", cpus.len());
        println!("   |- Global Usage: {:.1}%", global_cpu_usage);
    }

    println!("   |- Core Details:");
    for (i, cpu) in cpus.iter().enumerate() {
        println!(
            "      Core {:2} | Usage: {:5.1}% | Freq: {} MHz",
            i,
            cpu.cpu_usage(),
            cpu.frequency()
        );
    }
    println!("------------------------------------------");

    let disks = Disks::new_with_refreshed_list();
    println!("DISK STORAGE:");
    for disk in &disks {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        let used_space = total_space - available_space;
        let disk_percent = if total_space > 0 {
            used_space as f64 / total_space as f64 * 100.0
        } else {
            0.0
        };

        println!("   |- Mount Point: {:?}", disk.mount_point());
        println!("      |- File System: {:?}", disk.file_system());
        println!("      |- Total Space: {:.2} GB", to_gb(total_space));
        println!(
            "      |- Used Space: {:.2} GB ({:.1}%)",
            to_gb(used_space),
            disk_percent
        );
    }
    println!("==========================================");

    Ok(())
}
