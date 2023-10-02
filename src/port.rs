#[derive(Debug)]
pub struct Port {
    rw: bool,
    deviceaddr: u16,
    byte: u8,
}
