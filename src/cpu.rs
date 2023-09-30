pub struct cpu {
    regs: [u8; 0xff],
    disk: Vec<u8>,
}

impl cpu {
    fn new() -> Self {
        Self {
            regs,
            disk,
        }
    }

    fn init(&self) {

    }
}
