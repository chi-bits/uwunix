use std::fs::File;
use std::io::{Read, Write};

#[path = "./config.rs"]
mod config;

fn binfilewrite(file_path: &str, data: &[u8]) {
    let mut file = File::create(file_path).expect("Failed to create the file");

    file.write_all(data).expect("Failed to write data to the file");
}


fn binfileread(path: &str, buffer: &mut Vec<u8>) {
    let f = File::open(path);
    f.expect("yay").read_to_end(buffer);
}

pub struct Disk {
    // i have no ideas qwqwq
    path: String,
    disk: Vec<u8>,
}

impl Disk {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            disk: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        let t = config::get_table("uwunix.toml");
        self.path = String::from(t["disk"]["path"].to_string())
            .trim_end_matches('"')
            .trim_start_matches('"')
            .to_string();

        let mut buf: Vec<u8> = Vec::new();
        binfileread(
            self.path
                .as_str()
                .trim_end_matches('"')
                .trim_start_matches('"'),
            &mut buf
        );
        self.disk = buf;
    }

    pub fn fill(&mut self) {
        let zeroes: &[u8] = &[0xff; 0xffff];
        binfilewrite(self.path.as_str(), zeroes);

        let mut buf: Vec<u8> = Vec::new();

        for i in zeroes {
            buf.push(*i);
        }

        self.disk = buf;
    }

    pub fn read(&self, addr: u16) -> u8 {
        return self.disk[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.disk[addr as usize] = val;
    }
}
