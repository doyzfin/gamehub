use serde::Serialize;

#[derive(Serialize)]
pub struct HwInfoStats {
    cpu_temp: Option<f32>,
    gpu_temp: Option<f32>,
    gpu_usage: Option<f32>,
    vram_used: Option<f32>,
    vram_total: Option<f32>,
}

#[tauri::command]
pub fn get_hwinfo_stats() -> Result<HwInfoStats, String> {
    #[cfg(target_os = "windows")]
    {
        // For production, this is where we would map the HWiNFO Shared Memory.
        // HWiNFO creates a shared memory region named "Global\\HWiNFO32" or "HWiNFO32"
        // We would use `winapi` or `windows` crates to OpenFileMappingA, MapViewOfFile, etc.
        // Returning None since actual shared memory reading requires a real HWiNFO instance.
        
        Ok(HwInfoStats {
            cpu_temp: None,
            gpu_temp: None,
            gpu_usage: None,
            vram_used: None,
            vram_total: None,
        })
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // Not supported on Mac/Linux
        Ok(HwInfoStats {
            cpu_temp: None,
            gpu_temp: None,
            gpu_usage: None,
            vram_used: None,
            vram_total: None,
        })
    }
}
