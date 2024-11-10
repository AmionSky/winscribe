use crate::util;

const MAX_LEVEL: usize = 8;

pub struct ResWriter {
    level: usize,
    padding: Vec<u8>,
    buffer: Vec<u8>,
}

impl ResWriter {
    pub fn new() -> Self {
        let mut buffer = Vec::with_capacity(2048);
        buffer.extend(b"#pragma code_page(65001)\n"); // UTF-8

        Self {
            level: 0,
            padding: b"\t".repeat(MAX_LEVEL),
            buffer,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }

    pub fn line<T: AsRef<str>>(&mut self, line: T) {
        self.add_padding();
        self.buffer.extend(line.as_ref().as_bytes());
        self.new_line();
    }

    pub fn new_line(&mut self) {
        self.buffer.push(b'\n');
    }

    pub fn begin(&mut self) {
        self.add_padding();
        self.change_level(1);
        self.buffer.extend(b"{\n");
    }

    pub fn end(&mut self) {
        self.change_level(-1);
        self.add_padding();
        self.buffer.extend(b"}\n");
    }

    pub fn block<T: AsRef<str>>(&mut self, name: T) {
        self.add_padding();
        self.buffer.extend(b"BLOCK \"");
        self.buffer.extend(name.as_ref().as_bytes());
        self.buffer.extend(b"\"\n");
        self.begin();
    }

    pub fn value_str<N: AsRef<str>, V: AsRef<str>>(&mut self, name: N, value: V) {
        self.add_padding();
        self.buffer.extend(b"VALUE \"");
        self.buffer.extend(name.as_ref().as_bytes());
        self.buffer.extend(b"\", \"");
        self.buffer.extend(util::escape(value.as_ref()).as_bytes());
        self.buffer.extend(b"\"\n");
    }

    pub fn value_raw<N: AsRef<str>, V: AsRef<str>>(&mut self, name: N, value: V) {
        self.add_padding();
        self.buffer.extend(b"VALUE \"");
        self.buffer.extend(name.as_ref().as_bytes());
        self.buffer.extend(b"\", ");
        self.buffer.extend(value.as_ref().as_bytes());
        self.new_line();
    }

    fn change_level(&mut self, delta: isize) {
        self.level = self.level.saturating_add_signed(delta).min(MAX_LEVEL);
    }

    fn add_padding(&mut self) {
        self.buffer.extend(&self.padding[..self.level]);
    }
}

impl Default for ResWriter {
    fn default() -> Self {
        Self::new()
    }
}
