use std::env;

mod disk;
mod cpu;

fn main() {
    println!("uwunix :3");
    
    let mut disk = disk::Disk::new();
    let mut cpu = cpu::Cpu::new();

    let args: Vec<String> = env::args().collect();

    disk.init();
    cpu.init();


    if args.len() > 1 {
        if args[1] == "new-disk" {
            println!("Filled Disk with 0x00");
            disk.fill();
        }
    }

    
}
