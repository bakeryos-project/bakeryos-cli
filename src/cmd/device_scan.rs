use crate::{
    helpers::device::{find_unbound_devices, scan_pci_devices, PciDevice},
    utils::style::{bold, cyan, dim, divider, green, print_header, red, yellow},
};
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

            print_header(
                "BakeryOS Hardware Scanner",
                Some(&format!(
                    "Discovered {} total PCI device(s){}",
                    devices.len(),
                    if unbound { " (filtered: unbound only)" } else { "" }
                )),
            );

            let mut filtered_devices: Vec<&PciDevice> = devices.iter().collect();
            if unbound {
                filtered_devices = find_unbound_devices(&devices);
            }

            for device in filtered_devices {
                let vendor = device.vendor_name.as_deref().unwrap_or("Unknown vendor");
                let dev_name = device.device_name.as_deref().unwrap_or("Unknown device");
                let class_desc = device.class_name.as_deref().unwrap_or("Unknown class");

                let driver_status = match device.driver.as_deref() {
                    Some(drv) => green(format!("Bound to [{drv}]")),
                    None => red("Not bound (No driver)"),
                };

                println!(
                    "{} {}",
                    cyan(bold(&device.address)),
                    bold(format!("{vendor} {dev_name}"))
                );
                println!("   ├─ {}: {} ({})", dim("Class"), class_desc, dim(&device.class));
                println!(
                    "   ├─ {}: {} | {}: {}",
                    dim("Vendor ID"),
                    yellow(&device.vendor_id),
                    dim("Device ID"),
                    yellow(&device.device_id)
                );
                println!("   └─ {}: {}\n", dim("Driver"), driver_status);
            }

            println!("{}", divider(56));

            Ok(())
        })
        .build()
}
