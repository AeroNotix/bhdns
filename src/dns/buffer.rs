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

    pub fn read_u8_at_offset(&mut self, offset: usize) -> u8 {
        self.buf[self.position + offset]
    }


    pub fn read_u16(&mut self) -> u16 {
        ((self.read_u8() as u16) << 8) | (self.read_u8() as u16)
    }

    pub fn read_u16_at_offset(&mut self, offset: usize) -> u16 {
        ((self.read_u8_at_offset(offset) as u16) << 8) | (self.read_u8_at_offset(offset+1) as u16)
    }

    pub fn peek_u8(&mut self) -> u8 {
        self.buf[self.position]
    }

    pub fn peek_u16(&mut self) -> u16 {
        ((self.peek_u8() as u16) << 8) | (self.peek_u8() as u16)
    }

    pub fn read_sized(&mut self, size: usize) -> Vec<u8> {
        let ret = &self.buf[self.position..self.position + size];
        self.position += size;
        ret.to_owned()
    }

    pub fn read_sized_at_offset(&mut self, size: usize, offset: usize) -> Vec<u8> {
        let ret = &self.buf[offset..offset + size];
        ret.to_owned()
    }

    pub fn read_name(&mut self) -> String {
        let mut name = String::new();
        let mut join = "";

        loop {
            let len = self.peek_u8();
            if len == 0 {
                break;
            }
            name.push_str(join);
            dbg!(len);
            if len & 0xC0 == 0xC0 {
                dbg!("here");
                // This is a compressed label, https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.4
                let label_offset = (self.read_u16() ^ (0xC0 as u16)) as usize;
                let label_size = self.read_u8_at_offset(label_offset);
                dbg!(label_size);
                let label = self.read_sized_at_offset(label_offset+2, label_size as usize);
                name.push_str(&String::from_utf8_lossy(&label));
            } else {
                let size = self.read_u8();
                let label = self.read_sized(size as usize);
                name.push_str(&String::from_utf8_lossy(label.as_slice()));
            }
            join = "."
        }
        return name.to_owned();
    }

    pub fn seek(&mut self, position: usize) {
        self.position = position;
    }
}
