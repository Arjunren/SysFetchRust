use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    // Update system info
    sys.refresh_all();

    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let host_name = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    
    // RAM is in bytes for sysinfo v0.30
    let total_mem = sys.total_memory() / 1024 / 1024; // MB
    let used_mem = sys.used_memory() / 1024 / 1024; // MB
    
    let cpus = sys.cpus().len();
    let cpu_brand = if cpus > 0 { sys.cpus()[0].brand() } else { "Unknown" };
    
    let user_name = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "user".to_string());

    let ascii_art = r#"
      __
 __  / /
/ / / /_
\ \/ __ \
 \ / / / /
  \_\_/_/
"#;

    let info = format!(
        "
{}@{}
-------------------
OS: {} {}
CPU: {} ({} cores)
RAM: {} MB / {} MB
        ",
        user_name, host_name,
        os_name, os_version,
        cpu_brand, cpus,
        used_mem, total_mem
    );

    let art_lines: Vec<&str> = ascii_art.lines().collect();
    let info_lines: Vec<&str> = info.lines().collect();

    let max_lines = std::cmp::max(art_lines.len(), info_lines.len());
    
    println!("");
    for i in 0..max_lines {
        let art_line = *art_lines.get(i).unwrap_or(&"");
        let info_line = *info_lines.get(i).unwrap_or(&"");
        println!("{:<15} {}", art_line, info_line);
    }
    println!("");
}
