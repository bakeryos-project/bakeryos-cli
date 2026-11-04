use std::{
    fs,
    net::{SocketAddr, TcpStream},
    path::Path,
    process::Command,
    time::Duration,
};
use sysinfo::{Disks, ProcessesToUpdate, System};

use crate::{
    helpers::device::{find_unbound_devices, scan_pci_devices},
    utils::style::{
        bold, busy_badge, dim, divider, fail_badge, green, pass_badge, print_header, red,
        warn_badge, yellow,
    },
};

pub enum PacmanLockStatus {
    NoLock,
    Active,
    StaleLocked,
}

pub fn check_pacman_status() -> Result<PacmanLockStatus, String> {
    let lock_path = Path::new("/var/lib/pacman/db.lck");

    if !lock_path.exists() {
        return Ok(PacmanLockStatus::NoLock);
    }

    let mut sys = System::new_all().map_err(|e| e.to_string())?;
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let package_managers = ["pacman", "yay", "paru", "pamac-daemon"];
    let is_running = sys.processes().values().any(|proc| {
        let name = proc.name().to_string_lossy();
        package_managers.iter().any(|&pm| name.contains(pm))
    });

    if is_running {
        Ok(PacmanLockStatus::Active)
    } else {
        Ok(PacmanLockStatus::StaleLocked)
    }
}

pub fn check_failed_services() -> Result<Vec<String>, String> {
    let output = Command::new("systemctl")
        .args([
            "list-units",
            "--failed",
            "--no-legend",
            "--no-pager",
            "--plain",
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let failed: Vec<String> = stdout
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                trimmed.split_whitespace().next().map(|s| s.to_string())
            }
        })
        .collect();

    Ok(failed)
}

pub enum KernelModuleStatus {
    Synchronized,
    RebootRequired(String),
}

pub fn check_kernel_modules() -> Result<KernelModuleStatus, String> {
    let kernel_ver = System::kernel_version().unwrap_or_else(|_| String::from("unknown"));
    let modules_path = format!("/usr/lib/modules/{kernel_ver}");

    if !Path::new(&modules_path).exists() {
        Ok(KernelModuleStatus::RebootRequired(kernel_ver))
    } else {
        Ok(KernelModuleStatus::Synchronized)
    }
}

pub fn check_crash_dumps() -> Vec<String> {
    let mut dumps = Vec::new();
    let pstore_dir = Path::new("/sys/fs/pstore");

    if let Ok(entries) = fs::read_dir(pstore_dir) {
        for entry in entries.flatten() {
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with("dmesg-") || filename.starts_with("console-ramoops") {
                dumps.push(filename);
            }
        }
    }

    dumps
}

pub struct HighUsageDisk {
    pub mount: String,
    pub percent: f64,
    pub avail_gb: f64,
}

pub fn check_storage_capacity() -> Result<Vec<HighUsageDisk>, String> {
    let disks = Disks::new_with_refreshed_list().map_err(|e| e.to_string())?;
    let mut high_usage = Vec::new();

    for disk in &disks {
        let total = disk.total_space();
        let avail = disk.available_space();
        if total == 0 {
            continue;
        }

        let used = total - avail;
        let percent = (used as f64 / total as f64) * 100.0;

        if percent >= 85.0 {
            high_usage.push(HighUsageDisk {
                mount: disk.mount_point().to_string_lossy().to_string(),
                percent,
                avail_gb: avail as f64 / (1024.0 * 1024.0 * 1024.0),
            });
        }
    }

    Ok(high_usage)
}

pub fn check_network_connectivity() -> bool {
    let targets = [
        "1.1.1.1:53".parse::<SocketAddr>().unwrap(),
        "8.8.8.8:53".parse::<SocketAddr>().unwrap(),
    ];

    for target in &targets {
        if TcpStream::connect_timeout(target, Duration::from_millis(1500)).is_ok() {
            return true;
        }
    }

    false
}

pub fn diagnostic() -> Result<(), String> {
    print_header(
        "BakeryOS System Diagnostics",
        Some("Executing comprehensive system health audit across critical subsystems..."),
    );

    let mut ok_count = 0;
    let mut warn_count = 0;
    let mut error_count = 0;

    match check_pacman_status()? {
        PacmanLockStatus::NoLock => {
            println!(" {}  {}", pass_badge(), bold("Package Database Lock"));
            println!(
                "         {}",
                dim("└─ Database lock is free and accessible.")
            );
            ok_count += 1;
        }
        PacmanLockStatus::Active => {
            println!(" {}  {}", busy_badge(), bold("Package Database Lock"));
            println!(
                "         {}",
                dim("└─ A package manager process is currently active.")
            );
            warn_count += 1;
        }
        PacmanLockStatus::StaleLocked => {
            println!(" {}  {}", fail_badge(), bold("Package Database Lock"));
            println!(
                "         {}",
                red("├─ Found orphaned lock file: /var/lib/pacman/db.lck")
            );
            println!(
                "         {}",
                yellow("└─ Suggestion: Remove stale lock via 'sudo rm /var/lib/pacman/db.lck'")
            );
            error_count += 1;
        }
    }

    match check_failed_services() {
        Ok(failed) if failed.is_empty() => {
            println!(" {}  {}", pass_badge(), bold("Systemd Unit Services"));
            println!(
                "         {}",
                dim("└─ All system units loaded without failures.")
            );
            ok_count += 1;
        }
        Ok(failed) => {
            println!(" {}  {}", fail_badge(), bold("Systemd Unit Services"));
            println!(
                "         {}",
                red(format!("├─ Found {} degraded unit(s):", failed.len()))
            );
            for (i, svc) in failed.iter().enumerate() {
                let is_last = i == failed.len() - 1;
                let branch = if is_last { "└─" } else { "├─" };
                println!("         {}   {branch} {}", red("│"), red(bold(svc)));
            }
            println!(
                "         {}",
                yellow("└─ Suggestion: Inspect via 'journalctl -u <service> -e'")
            );
            error_count += 1;
        }
        Err(e) => {
            println!(" {}  {}", warn_badge(), bold("Systemd Unit Services"));
            println!(
                "         {}",
                dim(format!("└─ Unable to inspect systemd: {e}"))
            );
            warn_count += 1;
        }
    }

    match check_kernel_modules()? {
        KernelModuleStatus::Synchronized => {
            println!(" {}  {}", pass_badge(), bold("Kernel & Module Tree"));
            println!(
                "         {}",
                dim("└─ Running kernel matches installed module tree.")
            );
            ok_count += 1;
        }
        KernelModuleStatus::RebootRequired(ver) => {
            println!(" {}  {}", warn_badge(), bold("Kernel & Module Tree"));
            println!(
                "         {}",
                yellow(format!(
                    "├─ Running kernel ({ver}) modules no longer exist on disk."
                ))
            );
            println!(
                "         {}",
                yellow("└─ Suggestion: A kernel update was installed. Reboot is required.")
            );
            warn_count += 1;
        }
    }

    let crash_dumps = check_crash_dumps();
    if crash_dumps.is_empty() {
        println!(" {}  {}", pass_badge(), bold("Hardware Panic & Crash Logs"));
        println!(
            "         {}",
            dim("└─ No persistent crash logs found in pstore.")
        );
        ok_count += 1;
    } else {
        println!(" {}  {}", warn_badge(), bold("Hardware Panic & Crash Logs"));
        println!(
            "         {}",
            yellow(format!(
                "├─ Detected {} previous crash log(s) in /sys/fs/pstore:",
                crash_dumps.len()
            ))
        );
        for dump in &crash_dumps {
            println!("         {}   └─ {}", yellow("│"), yellow(dump));
        }
        println!(
            "         {}",
            yellow("└─ Suggestion: Review logs in /sys/fs/pstore/ or clear if resolved.")
        );
        warn_count += 1;
    }

    match scan_pci_devices() {
        Ok(devices) => {
            let unbound = find_unbound_devices(&devices);
            if unbound.is_empty() {
                println!(" {}  {}", pass_badge(), bold("PCI Device Drivers"));
                println!(
                    "         {}",
                    dim("└─ All detected PCI hardware devices are bound to kernel drivers.")
                );
                ok_count += 1;
            } else {
                println!(" {}  {}", warn_badge(), bold("PCI Device Drivers"));
                println!(
                    "         {}",
                    yellow(format!(
                        "├─ Found {} PCI device(s) without an active kernel driver:",
                        unbound.len()
                    ))
                );
                for (i, dev) in unbound.iter().enumerate() {
                    let is_last = i == unbound.len() - 1;
                    let branch = if is_last { "└─" } else { "├─" };
                    let vendor = dev.vendor_name.as_deref().unwrap_or("Unknown Vendor");
                    let name = dev.device_name.as_deref().unwrap_or("Unknown Device");
                    println!(
                        "         {}   {branch} {} [{vendor} {name}]",
                        yellow("│"),
                        bold(&dev.address)
                    );
                }
                println!(
                    "         {}",
                    yellow(
                        "└─ Suggestion: Check firmware packages or kernel modules (e.g. linux-firmware)."
                    )
                );
                warn_count += 1;
            }
        }
        Err(e) => {
            println!(" {}  {}", warn_badge(), bold("PCI Device Drivers"));
            println!(
                "         {}",
                dim(format!("└─ Unable to inspect PCI bus: {e}"))
            );
            warn_count += 1;
        }
    }

    match check_storage_capacity() {
        Ok(high_usage) if high_usage.is_empty() => {
            println!(" {}  {}", pass_badge(), bold("Storage Disk Capacity"));
            println!(
                "         {}",
                dim("└─ All mounted filesystems have sufficient headroom (<85%).")
            );
            ok_count += 1;
        }
        Ok(high_usage) => {
            println!(" {}  {}", warn_badge(), bold("Storage Disk Capacity"));
            for (i, d) in high_usage.iter().enumerate() {
                let is_last = i == high_usage.len() - 1;
                let branch = if is_last { "└─" } else { "├─" };
                println!(
                    "         {} {branch} Volume '{}' is at {:.1}% capacity ({:.2} GB left)",
                    yellow("│"),
                    bold(&d.mount),
                    d.percent,
                    d.avail_gb
                );
            }
            warn_count += 1;
        }
        Err(e) => {
            println!(" {}  {}", warn_badge(), bold("Storage Disk Capacity"));
            println!(
                "         {}",
                dim(format!("└─ Unable to query storage metrics: {e}"))
            );
            warn_count += 1;
        }
    }

    if check_network_connectivity() {
        println!(" {}  {}", pass_badge(), bold("Network Connectivity"));
        println!(
            "         {}",
            dim("└─ Outbound DNS connectivity verified successfully.")
        );
        ok_count += 1;
    } else {
        println!(" {}  {}", warn_badge(), bold("Network Connectivity"));
        println!(
            "         {}",
            yellow("├─ Failed to establish outbound test connection.")
        );
        println!(
            "         {}",
            yellow("└─ Suggestion: Check network interface or cable/Wi-Fi connection.")
        );
        warn_count += 1;
    }

    println!("\n{}", divider(56));
    print!("{} ", bold("Diagnostic Result:"));
    print!("{}, ", green(format!("{ok_count} passed")));
    print!("{}, ", yellow(format!("{warn_count} warning(s)")));
    println!("{}", red(format!("{error_count} error(s)")));

    if error_count == 0 && warn_count == 0 {
        println!(
            "{}\n",
            green(bold(
                "System audit passed. All critical subsystems operating normally."
            ))
        );
    } else if error_count > 0 {
        println!(
            "{}\n",
            red(bold(
                "System issues detected. Review recommendations above to restore optimal state."
            ))
        );
    } else {
        println!(
            "{}\n",
            yellow(bold(
                "Audit completed with warnings. System is functional but requires attention."
            ))
        );
    }

    Ok(())
}
