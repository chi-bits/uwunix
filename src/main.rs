use std::env;

mod disk;

fn main() {
    println!("uwunix :3");
    
    let mut disk = disk::Disk::new();
    let args: Vec<String> = env::args().collect();

    disk.init();

    if args[1] == "new-disk" {
        disk.fill();
    }
}
