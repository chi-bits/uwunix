use crate::disk::Disk;

#[path ="./iocat.rs"]
mod iocat;

pub struct Cpu {
    regs: [u8; 0xff],
    disk: Disk,
    pc: u64,
    pcbuf: u64,
    instrbuf: Vec<u8>,
    ram: [u8; 0xffff],
    port: iocat::Port,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: [0; 0xff],
            disk: Disk::new(),
            pc: 0,
            pcbuf: 0,
            instrbuf: Vec::new(),
            ram: [0; 0xffff],
            port: iocat::Port {
                rw: false,
                deviceaddr: 0x0000,
                byte: 0x00,
            }
        }
    }

    pub fn init(&mut self) {
        self.disk.init();
        self.pc = 0;
        
        // for i in 0..self.regs.len() {
        //     self.regs[i] = 0;
        // }
        //
        // for i in 0..self.ram.len() {
        //     self.ram[i] = 0;
        // }


        println!("PC: {}\nRegs{:?}", self.pc, self.regs);
    }

    fn clock(&mut self) {
        
    }
}
