use sysinfo::{Disks, System};

use crate::utils::{
    command::create_command,
    style::{
        bold, cyan, dim, divider, green, print_header, print_kv, print_kv_last, print_section,
        progress_bar, yellow,
    },
};

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
    let mut sys = System::new_all().map_err(|e| e.to_string())?;
    sys.refresh_all();

    let os_name = System::name().unwrap_or_else(|_| "Unknown OS".to_string());
    let os_version = System::os_version().unwrap_or_else(|_| "Unknown Version".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|_| "Unknown Kernel".to_string());
    let host_name = System::host_name().unwrap_or_else(|_| "Unknown Host".to_string());
    let uptime = System::uptime().map_err(|e| e.to_string())?;
    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;

    print_header(
        "BakeryOS System Status",
        Some("Real-time hardware utilization and operating system metrics"),
    );

    print_section("SYSTEM OVERVIEW");
    print_kv("Host", &host_name);
    print_kv("OS", format!("{os_name} ({os_version})"));
    print_kv("Kernel", &kernel_version);
    print_kv_last("Uptime", format!("{hours}h {minutes}m {seconds}s"));

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

    print_section("MEMORY & SWAP");
    print_kv(
        "RAM Usage",
        format!(
            " {}  {:.2} GB / {:.2} GB ({})",
            progress_bar(mem_percent, 16),
            to_gb(used_mem),
            to_gb(total_mem),
            green(format!("{:.2} GB free", to_gb(free_mem)))
        ),
    );
    print_kv_last(
        "Swap Usage",
        format!(
            "{}  {:.2} GB / {:.2} GB ({})",
            progress_bar(swap_percent, 16),
            to_gb(used_swap),
            to_gb(total_swap),
            dim(format!("{:.2} GB free", to_gb(free_swap)))
        ),
    );

    let cpus = sys.cpus();
    let global_cpu_usage = sys.global_cpu_usage();
    print_section("CPU PERFORMANCE");
    if let Some(first_cpu) = cpus.first() {
        print_kv("Model", first_cpu.brand());
        print_kv("Frequency", format!("{} MHz", first_cpu.frequency()));
        print_kv("Cores", format!("{} total cores", cpus.len()));
        print_kv(
            "Global Usage",
            format!(
                "{} ({} MHz)",
                progress_bar(global_cpu_usage as f64, 16),
                first_cpu.frequency()
            ),
        );
    }

    println!("\n   {}", dim("Per-Core Utilization:"));
    for (i, cpu) in cpus.iter().enumerate() {
        println!(
            "     {}  {}  {}",
            cyan(format!("Core {:2}", i)),
            progress_bar(cpu.usage() as f64, 10),
            dim(format!("{} MHz", cpu.frequency()))
        );
    }

    let disks = Disks::new_with_refreshed_list().map_err(|e| e.to_string())?;
    let valid_disks: Vec<_> = disks.iter().filter(|d| d.total_space() > 0).collect();

    print_section("STORAGE DISKS");
    for (i, disk) in valid_disks.iter().enumerate() {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        let used_space = total_space - available_space;
        let disk_percent = if total_space > 0 {
            used_space as f64 / total_space as f64 * 100.0
        } else {
            0.0
        };

        let is_last = i == valid_disks.len().saturating_sub(1);
        let branch = if is_last { "└─" } else { "├─" };

        let mount_point = disk.mount_point().to_string_lossy();
        let fs_type = disk.file_system().to_string_lossy();

        println!(
            "   {} {} ({})",
            cyan(branch),
            bold(&mount_point),
            yellow(&fs_type)
        );
        println!(
            "      {} {:.2} GB / {:.2} GB ({})",
            progress_bar(disk_percent, 16),
            to_gb(used_space),
            to_gb(total_space),
            green(format!("{:.2} GB avail", to_gb(available_space)))
        );
    }

    println!("\n{}\n", divider(56));

    Ok(())
}
