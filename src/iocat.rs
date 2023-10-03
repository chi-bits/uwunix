pub struct Port {
    pub rw: bool, // write = true, read = false 
    pub devaddr: u16,
    pub byte: u8,
}

impl Port {
    pub fn read(&mut self, addr: u16, byt: u8) {
        self.rw = false;
        self.devaddr = addr;
        self.byte = byt;
    }
}
