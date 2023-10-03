type block = [u8; 4];

pub struct Header {
    fname: &str,
    staddr: u32, // start address
    blockcount: u32,
}

impl Header {
    pub fn getlen(&self) -> usize {
        8 + self.fname.length()
    }
}

pub struct Dir {
    heads: Vec<Header>, 
    blocks: Vec<block>,
}

impl Dir {
    pub fn newfile(&mut self, fname: &str, data: &[u8]) {
        
    }
}


