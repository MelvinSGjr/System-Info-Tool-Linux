use std::process::Command;
use gethostname::gethostname;
use sysinfo::{Disks, System};

fn main() {
   println!("Systeam Info Checker");
   println!("created by MelvinSGjr");
   println!("!!!BETA TEST!!!");
   println!("__________________________");
   println!("");
    
    // OS name
    let sys: System = System::new_all();
    println!("OS: {:?}", sysinfo::System::name());

    // Hostname
    let hostname = gethostname().into_string().unwrap();
    println!("Hostname: {}", hostname);
    
    // Kernel info
    let output = Command::new("uname")
        .arg("-a")
        .output()
        .unwrap();
    println!("Kernel: {}", String::from_utf8_lossy(&output.stdout));

    // CPU core
    println!("CPU Cores: {}", sys.cpus().len());

    // RAM info
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("Total RAM: {} MB", sys.total_memory() / 1024 / 1024);
    println!("Used RAM: {} MB", sys.used_memory() / 1024 / 1024);

    // Disk info
    println!("");
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        println!("Disk: {:?}", disk.name());
    }
}