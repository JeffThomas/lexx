use crate::input::{Input, BUFFER_SIZE};

pub struct InputString {
    index: usize,
    size: usize,
    chars: Box<[char; BUFFER_SIZE]>,
}

impl InputString {
    pub fn new(text: String) -> Self {
        let mut chars = Box::new(['x'; BUFFER_SIZE]);
        let mut size: usize = 0;
        let cs = text.chars();
        for c in cs {
            chars[size] = c;
            size += 1;
            if size == BUFFER_SIZE {
                break;
            }
        }
        InputString {
            index: 0,
            size,
            chars,
        }
    }
}

impl Input for InputString {
    fn next(&mut self) -> Option<char> {
        if self.index < self.size {
            let c = self.chars[self.index];
            self.index += 1;
            return Some(c);
        }
        return None;
    }
}
