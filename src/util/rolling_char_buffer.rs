use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RollingBufferError {
    BufferFullError,
    BufferEmptyError,
}

impl std::error::Error for RollingBufferError {}

impl fmt::Display for RollingBufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RollingBufferError::BufferFullError => write!(f, "Buffer is full"),
            RollingBufferError::BufferEmptyError => write!(f, "Buffer is empty"),
        }
    }
}

pub struct RollingCharBuffer<const CAP: usize> {
    pub(crate) full: bool,
    pub(crate) cap: usize,

    start: usize,
    end: usize,
    buffer: Box<[char; CAP]>,
}

impl<const CAP: usize> RollingCharBuffer<CAP> {
    pub fn new() -> Self {
        RollingCharBuffer {
            start: 0,
            end: 0,
            full: false,
            cap: CAP,
            buffer: Box::new(['x'; CAP]),
        }
    }

    pub fn len(&self) -> usize {
        if self.full {
            self.cap
        } else {
            if self.end >= self.start {
                self.end - self.start
            } else {
                self.cap - (self.start - self.end)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        !(self.start != self.end || self.full)
    }

    pub fn clear(&mut self) {
        self.full = false;
        self.start = 0;
        self.end = 0;
    }

    pub fn push(&mut self, c: char) -> Result<(), RollingBufferError> {
        if self.full {
            return Err(RollingBufferError::BufferFullError);
        }
        self.buffer[self.end] = c;
        self.end += 1;
        if self.end == self.cap {
            self.end = 0;
        }
        if self.end == self.start {
            self.full = true;
        }
        Ok(())
    }

    pub fn pop(&mut self) -> Result<char, RollingBufferError> {
        if self.end == self.start && !self.full {
            return Err(RollingBufferError::BufferEmptyError);
        }
        let c = self.buffer[self.start];
        self.start += 1;
        if self.start == self.cap {
            self.start = 0
        }
        if self.full {
            self.full = false
        }
        Ok(c)
    }

    pub fn extend(&mut self, cs: &[char]) -> Result<usize, RollingBufferError> {
        if self.full || cs.len() > self.cap - self.len() {
            return Err(RollingBufferError::BufferFullError);
        }
        for c in cs {
            if let Err(e) = self.push(c.clone()) {
                return Err(e);
            }
        }
        Ok(self.cap - self.len())
    }
}

#[cfg(test)]
mod tests {
    use crate::util::rolling_char_buffer::RollingBufferError;
    use crate::RollingCharBuffer;

    #[test]
    fn test_buffer_is_empty() {
        let mut rb = RollingCharBuffer::<5>::new();

        assert_eq!(rb.len(), 0);
        assert_eq!(rb.is_empty(), true);
        assert_eq!(rb.pop(), Err(RollingBufferError::BufferEmptyError));
    }

    #[test]
    fn test_buffer_stores_one_char() {
        let mut rb = RollingCharBuffer::<5>::new();

        assert_eq!(rb.push('t'), Ok(()));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.is_empty(), false);
        assert_eq!(rb.pop(), Ok('t'));
        assert_eq!(rb.pop(), Err(RollingBufferError::BufferEmptyError));
        assert_eq!(rb.is_empty(), true);
    }

    #[test]
    fn test_buffer_fill() {
        let mut rb = RollingCharBuffer::<5>::new();
        assert_eq!(rb.push('a'), Ok(()));
        assert_eq!(rb.push('b'), Ok(()));
        assert_eq!(rb.push('c'), Ok(()));
        assert_eq!(rb.push('d'), Ok(()));
        assert_eq!(rb.push('e'), Ok(()));
        assert_eq!(rb.push('f'), Err(RollingBufferError::BufferFullError));
        assert_eq!(rb.is_empty(), false);
        assert_eq!(rb.len(), 5);
        assert_eq!(rb.pop(), Ok('a'));
        assert_eq!(rb.push('f'), Ok(()));
        assert_eq!(rb.pop(), Ok('b'));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.is_empty(), false);
    }

    #[test]
    fn test_buffer_clear() {
        let mut rb = RollingCharBuffer::<5>::new();
        assert_eq!(rb.push('a'), Ok(()));
        assert_eq!(rb.push('b'), Ok(()));
        assert_eq!(rb.push('c'), Ok(()));
        assert_eq!(rb.push('d'), Ok(()));
        assert_eq!(rb.push('e'), Ok(()));
        assert_eq!(rb.push('f'), Err(RollingBufferError::BufferFullError));
        assert_eq!(rb.len(), 5);
        assert_eq!(rb.is_empty(), false);
        rb.clear();
        assert_eq!(rb.is_empty(), true);
        assert_eq!(rb.pop(), Err(RollingBufferError::BufferEmptyError));
        assert_eq!(rb.len(), 0);
        assert_eq!(rb.push('a'), Ok(()));
        assert_eq!(rb.push('b'), Ok(()));
        assert_eq!(rb.push('c'), Ok(()));
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.pop(), Ok('a'));
        assert_eq!(rb.len(), 2);
        assert_eq!(rb.is_empty(), false);
        rb.clear();
        assert_eq!(rb.is_empty(), true);
        assert_eq!(rb.pop(), Err(RollingBufferError::BufferEmptyError));
        assert_eq!(rb.len(), 0);
        assert_eq!(rb.push('a'), Ok(()));
        assert_eq!(rb.len(), 1);
    }

    #[test]
    fn test_buffer_rolls() {
        let mut rb = RollingCharBuffer::<5>::new();
        assert_eq!(rb.push('a'), Ok(()));
        assert_eq!(rb.push('b'), Ok(()));
        assert_eq!(rb.push('c'), Ok(()));
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.pop(), Ok('a'));
        assert_eq!(rb.pop(), Ok('b'));
        assert_eq!(rb.pop(), Ok('c'));
        assert_eq!(rb.len(), 0);
        assert_eq!(rb.push('d'), Ok(()));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.push('e'), Ok(()));
        assert_eq!(rb.len(), 2);
        assert_eq!(rb.is_empty(), false);
        assert_eq!(rb.push('f'), Ok(()));
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.is_empty(), false);
        assert_eq!(rb.push('g'), Ok(()));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.push('h'), Ok(()));
        assert_eq!(rb.len(), 5);
        assert_eq!(rb.push('i'), Err(RollingBufferError::BufferFullError));
        assert_eq!(rb.is_empty(), false);
        assert_eq!(rb.pop(), Ok('d'));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.push('i'), Ok(()));
        assert_eq!(rb.len(), 5);
        assert_eq!(rb.pop(), Ok('e'));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.pop(), Ok('f'));
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.pop(), Ok('g'));
        assert_eq!(rb.len(), 2);
        assert_eq!(rb.pop(), Ok('h'));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.pop(), Ok('i'));
        assert_eq!(rb.len(), 0);
        assert_eq!(rb.pop(), Err(RollingBufferError::BufferEmptyError));
        assert_eq!(rb.is_empty(), true);
        assert_eq!(rb.push('j'), Ok(()));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.is_empty(), false);
    }

    #[test]
    fn test_buffer_extends() {
        let mut rb = RollingCharBuffer::<5>::new();

        assert_eq!(rb.push('a'), Ok(()));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.extend(&vec!['b', 'c', 'd']), Ok(1));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.pop(), Ok('a'));
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.pop(), Ok('b'));
        assert_eq!(rb.len(), 2);
        assert_eq!(rb.pop(), Ok('c'));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.extend(&vec!['e', 'f', 'g', 'h']), Ok(0));
        assert_eq!(rb.len(), 5);
        assert_eq!(
            rb.extend(&vec!['i', 'j', 'k', 'l']),
            Err(RollingBufferError::BufferFullError)
        );
        assert_eq!(rb.pop(), Ok('d'));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.pop(), Ok('e'));
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.pop(), Ok('f'));
        assert_eq!(rb.len(), 2);
        assert_eq!(
            rb.extend(&vec!['i', 'j', 'k', 'l']),
            Err(RollingBufferError::BufferFullError)
        );
        assert_eq!(rb.pop(), Ok('g'));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.pop(), Ok('h'));
        assert_eq!(rb.len(), 0);
        assert_eq!(rb.is_empty(), true);
    }
}
