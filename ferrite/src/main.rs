use sysinfo::System;
use std::time::Duration;

fn convert(bytes: f64) -> f64 {
    bytes / 1_073_741_824.0
}
fn main() {

    let mut sys = System::new_all();

    println!("---Ferrite System Info---");
    println!("System name: {}", System::name().unwrap_or("Unknown".to_string()));
    println!("System kernel version: {}", System::kernel_version().unwrap_or("Unknown".to_string()));
    println!("Total memory: {:.2} GB", convert(sys.total_memory() as f64));
    println!("Used memory: {:.2} GB", convert(sys.used_memory() as f64));
    println!("CPUs: {}", sys.cpus().len());

    let cpu_count = sys.cpus().len();

    loop {
        sys.refresh_cpu_usage();
        sys.refresh_memory();
        let avg = sys.cpus()    
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>()
            / cpu_count as f32;
            print!("\r{}% ", avg);
        std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
        std::thread::sleep(Duration::from_secs(1));}

}
