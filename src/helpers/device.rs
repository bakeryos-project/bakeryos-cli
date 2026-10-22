use std::{fs, io, path::Path};

use pciid_parser::Database;

#[derive(Debug, Clone)]
pub struct PciDevice {
    pub address: String,

    // Raw PCI identifiers
    pub vendor_id: String,
    pub device_id: String,
    pub class: String,

    // Human-readable names
    pub vendor_name: Option<String>,
    pub device_name: Option<String>,
    pub class_name: Option<String>,

    pub driver: Option<String>,
}

fn read_attr(path: &Path, name: &str) -> io::Result<String> {
    Ok(fs::read_to_string(path.join(name))?.trim().to_owned())
}

fn parse_hex(value: &str) -> Option<u32> {
    let value = value.strip_prefix("0x").unwrap_or(value);
    u32::from_str_radix(value, 16).ok()
}

fn resolve_names(
    db: Option<&Database>,
    vendor_id: &str,
    device_id: &str,
    class: &str,
) -> (Option<String>, Option<String>, Option<String>) {
    let Some(db) = db else {
        return (None, None, None);
    };

    let vendor_id = parse_hex(vendor_id);
    let device_id = parse_hex(device_id);
    let class_code = parse_hex(class);

    let vendor_name = vendor_id
        .and_then(|id| u16::try_from(id).ok())
        .and_then(|id| db.vendors.get(&id))
        .map(|vendor| vendor.name.clone());

    let device_name = vendor_id
        .and_then(|id| u16::try_from(id).ok())
        .and_then(|vendor_id| db.vendors.get(&vendor_id))
        .and_then(|vendor| {
            device_id
                .and_then(|id| u16::try_from(id).ok())
                .and_then(|device_id| vendor.devices.get(&device_id))
        })
        .map(|device| device.name.clone());

    // PCI class format: base class (8 bits),
    // subclass (8 bits), programming interface (8 bits).
    let class_name = class_code
        .and_then(|code| u8::try_from((code >> 16) & 0xff).ok())
        .and_then(|base_class| db.classes.get(&base_class))
        .map(|class| class.name.clone());

    (vendor_name, device_name, class_name)
}

pub fn scan_pci_devices() -> io::Result<Vec<PciDevice>> {
    let root = Path::new("/sys/bus/pci/devices");
    let mut devices = Vec::new();

    // If the local PCI database is unavailable, scanning still works.
    let database = Database::read().ok();

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();

        let vendor_id = read_attr(&path, "vendor")?;
        let device_id = read_attr(&path, "device")?;
        let class = read_attr(&path, "class")?;

        let (vendor_name, device_name, class_name) =
            resolve_names(database.as_ref(), &vendor_id, &device_id, &class);

        let driver_path = path.join("driver");

        let driver = fs::read_link(&driver_path).ok().and_then(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
        });

        devices.push(PciDevice {
            address: entry.file_name().to_string_lossy().into_owned(),
            vendor_id,
            device_id,
            class,
            vendor_name,
            device_name,
            class_name,
            driver,
        });
    }

    Ok(devices)
}

// pub fn find_devices_without_driver(devices: &[PciDevice]) -> Vec<&PciDevice> {
//     devices
//         .iter()
//         .filter(|device| device.driver.is_none())
//         .collect()
// }

pub fn find_unbound_devices(devices: &[PciDevice]) -> Vec<&PciDevice> {
    devices
        .iter()
        .filter(|device| device.driver.is_none())
        .collect()
}
