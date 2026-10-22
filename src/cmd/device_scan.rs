use crate::helpers::device::{PciDevice, find_unbound_devices, scan_pci_devices};
use flag_rs::{Command, CommandBuilder, Flag, FlagType, FlagValue};

pub fn init() -> Command {
    CommandBuilder::new("scan")
        .flag(
            Flag::new("unbound")
                .short('u')
                .usage("Show unbound device")
                .value_type(FlagType::Bool)
                .default(FlagValue::Bool(false)),
        )
        .short("Scan device")
        .run(|ctx| {
            let unbound = ctx
                .flag("unbound")
                .and_then(|s| s.parse::<bool>().ok())
                .unwrap_or(false);
            let devices = scan_pci_devices()?;
            println!("Detected {} PCI devices", devices.len());

            let mut filtered_devices: Vec<&PciDevice> = devices.iter().collect();
            if unbound {
                filtered_devices = find_unbound_devices(&devices);
            }

            for device in filtered_devices {
                println!(
                    "{}: {} {}",
                    device.address,
                    device.vendor_name.as_deref().unwrap_or("Unknown vendor"),
                    device.device_name.as_deref().unwrap_or("Unknown device"),
                );

                println!(
                    "  Class:  {} ({})",
                    device.class_name.as_deref().unwrap_or("Unknown class"),
                    device.class,
                );

                println!("  Vendor: {}", device.vendor_id);
                println!("  Device: {}", device.device_id);

                println!(
                    "  Driver: {}",
                    device.driver.as_deref().unwrap_or("Not bound"),
                );
            }

            Ok(())
        })
        .build()
}
