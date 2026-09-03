use serde::Serialize;
use sysinfo::{System, CpuRefreshKind, RefreshKind};
use std::sync::Mutex;

#[derive(Serialize)]
pub struct SystemStats {
    cpu_usage: f32,
    cpu_temperature: Option<f32>,
    gpu_usage: Option<f32>,
    gpu_temperature: Option<f32>,
    ram_used: f32,
    ram_total: f32,
    vram_used: Option<f32>,
    vram_total: Option<f32>,
}

pub struct SystemState {
    pub sys: Mutex<System>,
}

#[tauri::command]
pub fn get_system_stats(state: tauri::State<'_, SystemState>) -> SystemStats {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()).with_memory());
    
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let ram_used = sys.used_memory() as f32 / 1024.0 / 1024.0 / 1024.0;
    let ram_total = sys.total_memory() as f32 / 1024.0 / 1024.0 / 1024.0;
    
    // For MVP phase 1 without HWiNFO or heavy WMI calls, we return basic CPU/RAM.
    // GPU and CPU Temp require elevated APIs or sensors component integration.
    SystemStats {
        cpu_usage,
        cpu_temperature: None, // N/A
        gpu_usage: None,
        gpu_temperature: None,
        ram_used,
        ram_total,
        vram_used: None,
        vram_total: None,
    }
}
