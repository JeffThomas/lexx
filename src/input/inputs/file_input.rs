use crate::input::{Input, BUFFER_SIZE};
use std::fs::File;
use std::io::SeekFrom;
use std::io::{Read, Seek};
use std::str::{from_utf8, from_utf8_unchecked};

pub struct InputFile {
    index: usize,
    size: usize,
    file: File,
    buffer: Box<[u8; BUFFER_SIZE]>,
    text: Box<[char; BUFFER_SIZE]>,
}

impl InputFile {
    pub fn new(input: String) -> Self {
        let file = File::open(input).unwrap();
        let buffer = Box::new([0; BUFFER_SIZE]);
        let text = Box::new(['x'; BUFFER_SIZE]);

        InputFile {
            index: 1,
            size: 0,
            file,
            buffer,
            text,
        }
    }
}

impl Input for InputFile {
    fn next(&mut self) -> Option<char> {
        if self.index < self.size {
            let c = self.text[self.index];
            self.index += 1;
            return Some(c);
        }
        let n = self.file.read(self.buffer.as_mut()).unwrap();
        if n == 0 {
            return None;
        }
        let se: &str;
        {
            match from_utf8(&self.buffer[..n]) {
                Ok(s) => {
                    se = s;
                }
                Err(e) => {
                    let end = e.valid_up_to();
                    // This is safe due to the above check
                    se = unsafe { from_utf8_unchecked(&self.buffer[..n][..end]) };
                    let offset = (end - n) as i64;
                    // we could also just hold onto the bytes at the start of our buffer but this is a
                    // bit simpler IMO
                    self.file.seek(SeekFrom::Current(-1 * offset)).unwrap();
                }
            }
        }
        self.size = 0;
        self.index = 1;
        let cs = se.chars();
        for c in cs {
            self.text[self.size] = c;
            self.size += 1;
        }
        return Some(self.text[0]);
    }
}
