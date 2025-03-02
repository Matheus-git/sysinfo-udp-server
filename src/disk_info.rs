use serde::Serialize;
use serde_json::to_string_pretty;
use sysinfo::{Disks, Disk};

#[derive(Serialize, Debug)]
struct DiskInfo {
    name: String,
    mount_point: String,
    total_space: u64,
    available_space: u64,
    file_system: String,
}

impl From<&Disk> for DiskInfo {
    fn from(disk: &Disk) -> Self {
        DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            file_system: disk.file_system().to_string_lossy().to_string(),
        }
    }
}

pub fn disks_info_json() -> String {
    let disks = Disks::new_with_refreshed_list();
    let disk_info_list: Vec<DiskInfo> = disks.list().iter().map(|disk| DiskInfo::from(disk)).collect();

    match to_string_pretty(&disk_info_list) {
        Ok(json) => json, 
        Err(e) => {
            eprintln!("Error on serialize (disks_info_json): {}", e);
            String::new()
        }
    }
}