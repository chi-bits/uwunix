mod disk;
mod instr;

pub struct cpu {
    regs: [u8; 0xff],
    disk: disk::Disk,
    pc: u64,
    instrbuf: Vec<u8>,
}

impl cpu {
    fn new() -> Self {
        Self {
            regs,
            disk: disk::Disk::new(),
            pc,
        }
    }

    fn init(&mut self) {
        self.disk.init();
        self.pc = 0;

    }

    fn clock(&mut self) {

    }
}
