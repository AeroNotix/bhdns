pub struct Buffer {
    position: usize,
    buf: Vec<u8>,
}

impl Buffer {
    pub fn new(buf: Vec<u8>) -> Buffer {
        Buffer { position: 0, buf }
    }

    pub fn read_u8(&mut self) -> u8 {
        let value = self.buf[self.position];
        self.position += 1;
        value
    }
    pub fn read_u16(&mut self) -> u16 {
        ((self.read_u8() as u16) << 8) | (self.read_u8() as u16)
    }

    pub fn read_sized(&mut self, size: usize) -> &[u8] {
        let ret = &self.buf[self.position..self.position + size];
        self.position += size;
        ret
    }

    pub fn seek(&mut self, position: usize) {
        self.position = position;
    }
}
