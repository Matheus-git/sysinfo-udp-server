use serde::Serialize;
use serde_json::to_string_pretty;
use sysinfo::{System, Cpu};
use sysinfo::{RefreshKind, CpuRefreshKind};

#[derive(Serialize, Debug)]
struct CPUInfo {
    name: String,
    usage: f32,
    vendor: String,
    brand: String,
    frequency: u64,
}

impl From<&Cpu> for CPUInfo {
    fn from(cpu: &Cpu) -> Self {
        CPUInfo {
            name: cpu.name().to_string(),
            usage: cpu.cpu_usage(),
            vendor: cpu.vendor_id().to_string(),
            brand: cpu.brand().to_string(),
            frequency: cpu.frequency(),
        }
    }
}

pub fn cpus_info_json() -> String {
    let mut s = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );    
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    s.refresh_cpu_all(); 
    let cpus_info_list: Vec<CPUInfo> = s.cpus().iter().map(|cpu| CPUInfo::from(cpu)).collect(); 
    
    match to_string_pretty(&cpus_info_list) {
        Ok(json) => json, 
        Err(e) => {
            eprintln!("Error on serialize (cpus_info_json): {}", e);
            String::new()
        }
    }
}