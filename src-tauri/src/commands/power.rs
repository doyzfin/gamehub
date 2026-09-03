#[tauri::command]
pub fn power_action(action: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        match action.as_str() {
            "shutdown" => {
                Command::new("shutdown")
                    .args(["/s", "/t", "0"])
                    .spawn()
                    .map_err(|e| e.to_string())?;
                Ok(())
            },
            "restart" => {
                Command::new("shutdown")
                    .args(["/r", "/t", "0"])
                    .spawn()
                    .map_err(|e| e.to_string())?;
                Ok(())
            },
            "sleep" => {
                // Using rundll32 for sleep is common, though SetSuspendState API is better for production.
                // For MVP, this handles standard sleep.
                Command::new("rundll32.exe")
                    .args(["powrprof.dll,SetSuspendState", "0,1,0"])
                    .spawn()
                    .map_err(|e| e.to_string())?;
                Ok(())
            },
            _ => Err("Unknown power action".into())
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // Mock for mac/linux dev
        println!("Mock power action: {}", action);
        Ok(())
    }
}
