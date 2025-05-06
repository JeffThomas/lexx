//! # RollingCharBuffer
//!
//! A high-performance, fixed-size circular buffer for `char` values, supporting both FIFO (queue) and LIFO (stack) operations.
//!
//! This structure is designed for use in lexers, tokenizers, and other performance-critical text processing applications where rapid, bounded buffering of characters is required with minimal allocations.
//!
//! ## Features
//! - Constant-time push, pop, prefix, and read operations.
//! - Supports both stack-like (LIFO) and queue-like (FIFO) access patterns.
//! - Efficiently handles buffer wrap-around (circular/ring buffer design).
//! - Fixed capacity: no dynamic allocation or resizing after creation.
//! - Graceful error handling when buffer is full or empty.
//!
//! ## Performance
//! - All operations are O(1) and require no heap allocations after initialization.
//! - Suitable for high-throughput scenarios such as streaming tokenization or incremental parsing.
//!
//! ## Example Usage
//! ```rust
//! use lexx::rolling_char_buffer::{RollingCharBuffer, RollingCharBufferError};
//! let mut buffer = RollingCharBuffer::<8>::new();
//! buffer.push('x').unwrap();
//! buffer.prefix('y').unwrap();
//! assert_eq!(buffer.read().unwrap(), 'y');
//! assert_eq!(buffer.pop().unwrap(), 'x');
//! ```
//!
//! See method documentation and examples for more details.

use std::fmt;

/// RollingCharBuffer errors
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RollingCharBufferError {
    /// there is no room left in the buffer
    BufferFullError,
    /// you can't get anything out of an empty buffer
    BufferEmptyError,
}

impl std::error::Error for RollingCharBufferError {}

impl fmt::Display for RollingCharBufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RollingCharBufferError::BufferFullError => write!(f, "Buffer is full"),
            RollingCharBufferError::BufferEmptyError => write!(f, "Buffer is empty"),
        }
    }
}

/// RollingCharBuffer is a fast, fixed size [char] buffer that can be used as a LIFO or FIFO stack.
///
/// * [push](RollingCharBuffer::push) adds a [char] to the end of the buffer
/// * [pop](RollingCharBuffer::pop) returns and removes the [char] from the end of the buffer
/// * [prefix](RollingCharBuffer::prefix) adds a [char] to the front of buffer
/// * [read](RollingCharBuffer::read) returns and removes the [char] from the front of the buffer
/// * [extend](RollingCharBuffer::extend) adds a [vec]<[char]> to the end of the buffer
/// * [prepend](RollingCharBuffer::prepend) adds a [vec]<[char]> to the front of the buffer
/// * [clear](RollingCharBuffer::clear) clears the buffer
///
/// # Example
///
/// ```rust
/// use lexx::rolling_char_buffer::{RollingCharBuffer, RollingCharBufferError};
/// let mut buffer = RollingCharBuffer::<5>::new();
///
/// assert_eq!(buffer.push('a'), Ok(())); // buffer is now ['a']
/// assert_eq!(buffer.len(), 1);
/// assert_eq!(buffer.is_empty(), false);
/// assert_eq!(buffer.is_full(), false);
/// assert!(matches!(buffer.pop(), Ok(c) if c == 'a'));
/// assert_eq!(buffer.is_empty(), true);
/// assert_eq!(buffer.is_full(), false);
/// assert_eq!(buffer.pop(), Err(RollingCharBufferError::BufferEmptyError));
/// assert_eq!(buffer.push('b'), Ok(())); // buffer is now ['b']
/// assert_eq!(buffer.push('c'), Ok(())); // buffer is now ['b', 'c']
/// assert_eq!(buffer.prefix('a'), Ok(())); // buffer is now ['a', 'b', 'c']
/// assert!(matches!(buffer.read(), Ok(c) if c == 'a')); // buffer is now ['b', 'c']
/// assert_eq!(buffer.len(), 2);
/// assert_eq!(buffer.extend(&vec!['d', 'e', 'f']), Ok(0)); // buffer is now ['b', 'c', 'd', 'e', 'f']
/// assert_eq!(buffer.is_empty(), false);
/// assert_eq!(buffer.is_full(), true);
/// assert_eq!(buffer.len(), 5);
/// assert_eq!(buffer.extend(&vec!['g', 'h', 'i']), Err(RollingCharBufferError::BufferFullError));
/// assert_eq!(buffer.push('g'), Err(RollingCharBufferError::BufferFullError));
/// assert!(matches!(buffer.read(), Ok(c) if c == 'b')); // buffer is now ['c', 'd', 'e', 'f']
/// assert_eq!(buffer.len(), 4);
/// assert!(matches!(buffer.pop(), Ok(c) if c == 'f')); // buffer is now ['c', 'd', 'e']
/// assert_eq!(buffer.len(), 3);
/// assert_eq!(buffer.prepend(&vec!['a', 'b']), Ok(0)); // buffer is now ['a', 'b', 'c', 'd', 'e']
/// assert_eq!(buffer.is_full(), true);
/// assert!(matches!(buffer.read(), Ok(c) if c == 'a'));
/// assert!(matches!(buffer.pop(), Ok(c) if c == 'e'));
/// assert!(matches!(buffer.read(), Ok(c) if c == 'b'));
/// assert!(matches!(buffer.pop(), Ok(c) if c == 'd'));
/// assert!(matches!(buffer.read(), Ok(c) if c == 'c'));
/// assert_eq!(buffer.pop(), Err(RollingCharBufferError::BufferEmptyError));
/// assert_eq!(buffer.len(), 0);
/// assert_eq!(buffer.is_empty(), true);
/// assert_eq!(buffer.is_full(), false);
/// ```
///
#[derive(Debug)]
pub struct RollingCharBuffer<const CAP: usize> {
    pub(crate) full: bool,
    pub(crate) cap: usize,

    start: usize,
    end: usize,
    buffer: Box<[char; CAP]>,
}

impl<const CAP: usize> Default for RollingCharBuffer<CAP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAP: usize> RollingCharBuffer<CAP> {
    /// Creates a new [RollingCharBuffer]
    ///
    /// ```rust
    /// use lexx::rolling_char_buffer::RollingCharBuffer;
    /// // create a new [RollingCharBuffer] with a maximum buffer size of 5 [char]s
    /// let mut buffer = RollingCharBuffer::<5>::new();
    /// ```
    pub fn new() -> Self {
        RollingCharBuffer {
            start: CAP - 1,
            end: CAP - 1,
            full: false,
            cap: CAP,
            buffer: Box::new(['x'; CAP]),
        }
    }

    /// Returns how many [char]s are in the buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexx::rolling_char_buffer::RollingCharBuffer;
    ///
    /// let mut buffer = RollingCharBuffer::<5>::new();
    /// assert_eq!(buffer.len(), 0);
    /// assert_eq!(buffer.push('a'), Ok(())); // buffer is now ['a']
    /// assert_eq!(buffer.len(), 1);
    /// assert_eq!(buffer.push('b'), Ok(())); // buffer is now ['a', 'b']
    /// assert_eq!(buffer.len(), 2);
    /// assert!(matches!(buffer.pop(), Ok(c) if c == 'b')); // buffer is now ['a']
    /// assert_eq!(buffer.len(), 1);
    /// ```
    ///
    pub fn len(&self) -> usize {
        if self.full {
            self.cap
        } else if self.end >= self.start {
            self.end - self.start
        } else {
            self.cap - (self.start - self.end)
        }
    }

    /// Returns if the buffer is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexx::rolling_char_buffer::RollingCharBuffer;
    ///
    /// let mut buffer = RollingCharBuffer::<5>::new();
    /// assert_eq!(buffer.is_empty(), true);
    /// assert_eq!(buffer.push('a'), Ok(())); // buffer is now ['a']
    /// assert_eq!(buffer.is_empty(), false);
    /// assert!(matches!(buffer.pop(), Ok(c) if c == 'a')); // buffer is now []
    /// assert_eq!(buffer.is_empty(), true);
    /// ```
    ///
    pub fn is_empty(&self) -> bool {
        self.start == self.end && !self.full
    }
    /// Returns if the buffer is full
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexx::rolling_char_buffer::RollingCharBuffer;
    ///
    /// let mut buffer = RollingCharBuffer::<2>::new();
    /// assert_eq!(buffer.is_full(), false);
    /// assert_eq!(buffer.push('a'), Ok(())); // buffer is now ['a']
    /// assert_eq!(buffer.is_full(), false);
    /// assert_eq!(buffer.push('b'), Ok(())); // buffer is now ['a', 'b']
    /// assert_eq!(buffer.is_full(), true);
    /// assert!(matches!(buffer.pop(), Ok(c) if c == 'b')); // buffer is now ['a']
    /// assert_eq!(buffer.is_full(), false);
    /// ```
    ///
    pub fn is_full(&self) -> bool {
        self.full
    }
    /// Empties the buffer
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use lexx::rolling_char_buffer::RollingCharBuffer;
    /// let mut buffer = RollingCharBuffer::<5>::new();
    /// assert_eq!(buffer.push('a'), Ok(())); // buffer is now ['a']
    /// assert_eq!(buffer.push('b'), Ok(())); // buffer is now ['a', 'b']
    /// assert_eq!(buffer.len(), 2);
    /// buffer.clear();
    /// assert_eq!(buffer.len(), 0);
    /// ```
    ///
    pub fn clear(&mut self) {
        self.full = false;
        self.start = self.end;
    }

    /// Adds a [char] to the end of the buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexx::rolling_char_buffer::RollingCharBuffer;
    ///
    /// let mut buffer = RollingCharBuffer::<5>::new();
    /// assert_eq!(buffer.push('a'), Ok(())); // buffer is now ['a']
    /// assert_eq!(buffer.push('b'), Ok(())); // buffer is now ['a', 'b']
    /// ```
    ///
    pub fn push(&mut self, c: char) -> Result<(), RollingCharBufferError> {
        if self.full {
            return Err(RollingCharBufferError::BufferFullError);
        }
        self.buffer[self.end] = c;
        self.end = (self.end + 1) % self.cap;
        if self.end == self.start {
            self.full = true;
        }
        Ok(())
    }

    /// Reads and removes a [char] from the front of the buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexx::rolling_char_buffer::RollingCharBuffer;
    ///
    /// let mut buffer = RollingCharBuffer::<5>::new();
    /// assert_eq!(buffer.push('a'), Ok(())); // buffer is now ['a']
    /// assert_eq!(buffer.push('b'), Ok(())); // buffer is now ['a', 'b']
    /// assert!(matches!(buffer.read(), Ok(c) if c == 'a')); // buffer is now ['b']
    /// ```
    ///
    pub fn read(&mut self) -> Result<char, RollingCharBufferError> {
        if self.is_empty() {
            return Err(RollingCharBufferError::BufferEmptyError);
        }
        let c = self.buffer[self.start];
        self.start = (self.start + 1) % self.cap;
        if self.full {
            self.full = false;
        }
        Ok(c)
    }

    /// Reads and removes a [char] from the end of the buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexx::rolling_char_buffer::RollingCharBuffer;
    ///
    /// let mut buffer = RollingCharBuffer::<5>::new();
    /// assert_eq!(buffer.push('a'), Ok(())); // buffer is now ['a']
    /// assert_eq!(buffer.push('b'), Ok(())); // buffer is now ['a', 'b']
    /// assert!(matches!(buffer.pop(), Ok(c) if c == 'b')); // buffer is now ['a']
    /// ```
    ///
    pub fn pop(&mut self) -> Result<char, RollingCharBufferError> {
        if self.is_empty() {
            return Err(RollingCharBufferError::BufferEmptyError);
        }
        self.end = if self.end == 0 {
            self.cap - 1
        } else {
            self.end - 1
        };
        if self.full {
            self.full = false;
        }
        Ok(self.buffer[self.end])
    }

    /// Adds a [char] to the front of the buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexx::rolling_char_buffer::RollingCharBuffer;
    ///
    /// let mut buffer = RollingCharBuffer::<5>::new();
    /// assert_eq!(buffer.prefix('a'), Ok(())); // buffer is now ['a']
    /// assert_eq!(buffer.prefix('b'), Ok(())); // buffer is now ['b', 'a']
    /// ```
    ///
    pub fn prefix(&mut self, c: char) -> Result<(), RollingCharBufferError> {
        if self.full {
            return Err(RollingCharBufferError::BufferFullError);
        }
        self.start = if self.start == 0 {
            self.cap - 1
        } else {
            self.start - 1
        };
        self.buffer[self.start] = c;
        if self.end == self.start {
            self.full = true;
        }
        Ok(())
    }

    /// Adds a [vec]<[char]> to the end of the buffer
    ///
    /// # Arguments
    ///
    /// * `charvec` - The [vec]<[char]> to add to the buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexx::rolling_char_buffer::RollingCharBuffer;
    ///
    /// let mut buffer = RollingCharBuffer::<5>::new();
    /// assert_eq!(buffer.push('a'), Ok(())); // buffer is now ['a']
    /// assert_eq!(buffer.extend(&vec!['b', 'c', 'd']), Ok(1)); // buffer is now ['a', 'b', 'c', 'd']
    /// assert!(matches!(buffer.read(), Ok(c) if c == 'a')); // buffer is now ['b', 'c', 'd']
    /// assert!(matches!(buffer.read(), Ok(c) if c == 'b')); // buffer is now ['c', 'd']
    /// ```
    ///
    pub fn extend(&mut self, charvec: &[char]) -> Result<usize, RollingCharBufferError> {
        let free = self.cap - self.len();
        if self.full || charvec.len() > free {
            return Err(RollingCharBufferError::BufferFullError);
        }
        for &c in charvec {
            self.push(c)?;
        }
        Ok(self.cap - self.len())
    }

    /// Adds a [vec]<[char]> to the front of the buffer
    ///
    /// # Arguments
    ///
    /// * `charvec` - The [vec]<[char]> to add to the buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexx::rolling_char_buffer::RollingCharBuffer;
    ///
    /// let mut buffer = RollingCharBuffer::<5>::new();
    /// assert_eq!(buffer.push('a'), Ok(())); // buffer is now ['a']
    /// assert_eq!(buffer.prepend(&vec!['b', 'c', 'd']), Ok(1)); // buffer is now ['b', 'c', 'd', 'a']
    /// assert!(matches!(buffer.pop(), Ok(c) if c == 'a')); // buffer is now ['b', 'c', 'd']
    /// assert!(matches!
    /// (buffer.pop(), Ok(c) if c == 'd')); // buffer is now ['b', 'c']
    /// ```
    ///
    pub fn prepend(&mut self, cs: &[char]) -> Result<usize, RollingCharBufferError> {
        let free = self.cap - self.len();
        if self.full || cs.len() > free {
            return Err(RollingCharBufferError::BufferFullError);
        }
        for &c in cs.iter().rev() {
            self.prefix(c)?;
        }
        Ok(self.cap - self.len())
    }
}

#[cfg(test)]
mod tests {
    use crate::rolling_char_buffer::RollingCharBufferError;
    use crate::RollingCharBuffer;

    #[test]
    fn test_buffer_is_empty() {
        let mut rb = RollingCharBuffer::<5>::new();

        assert_eq!(rb.len(), 0);
        assert!(rb.is_empty());
        assert_eq!(rb.read(), Err(RollingCharBufferError::BufferEmptyError));
    }

    #[test]
    fn test_buffer_stores_one_char() {
        let mut rb = RollingCharBuffer::<5>::new();

        assert_eq!(rb.push('t'), Ok(()));
        assert_eq!(rb.len(), 1);
        assert!(!rb.is_empty());
        assert_eq!(rb.read(), Ok('t'));
        assert_eq!(rb.read(), Err(RollingCharBufferError::BufferEmptyError));
        assert!(rb.is_empty());
    }

    #[test]
    fn test_buffer_fill() {
        let mut rb = RollingCharBuffer::<5>::new();
        assert_eq!(rb.push('a'), Ok(()));
        assert_eq!(rb.push('b'), Ok(()));
        assert_eq!(rb.push('c'), Ok(()));
        assert_eq!(rb.push('d'), Ok(()));
        assert_eq!(rb.push('e'), Ok(()));
        assert_eq!(rb.push('f'), Err(RollingCharBufferError::BufferFullError));
        assert!(!rb.is_empty());
        assert_eq!(rb.len(), 5);
        assert_eq!(rb.read(), Ok('a'));
        assert_eq!(rb.push('f'), Ok(()));
        assert_eq!(rb.read(), Ok('b'));
        assert_eq!(rb.len(), 4);
        assert!(!rb.is_empty());
    }

    #[test]
    fn test_buffer_clear() {
        let mut rb = RollingCharBuffer::<5>::new();
        assert_eq!(rb.push('a'), Ok(()));
        assert_eq!(rb.push('b'), Ok(()));
        assert_eq!(rb.push('c'), Ok(()));
        assert_eq!(rb.push('d'), Ok(()));
        assert_eq!(rb.push('e'), Ok(()));
        assert_eq!(rb.push('f'), Err(RollingCharBufferError::BufferFullError));
        assert_eq!(rb.len(), 5);
        assert!(!rb.is_empty());
        rb.clear();
        assert!(rb.is_empty());
        assert_eq!(rb.read(), Err(RollingCharBufferError::BufferEmptyError));
        assert_eq!(rb.len(), 0);
        assert_eq!(rb.push('a'), Ok(()));
        assert_eq!(rb.push('b'), Ok(()));
        assert_eq!(rb.push('c'), Ok(()));
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.read(), Ok('a'));
        assert_eq!(rb.len(), 2);
        assert!(!rb.is_empty());
        rb.clear();
        assert!(rb.is_empty());
        assert_eq!(rb.read(), Err(RollingCharBufferError::BufferEmptyError));
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
        assert_eq!(rb.read(), Ok('a'));
        assert_eq!(rb.read(), Ok('b'));
        assert_eq!(rb.read(), Ok('c'));
        assert_eq!(rb.len(), 0);
        assert_eq!(rb.push('d'), Ok(()));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.push('e'), Ok(()));
        assert_eq!(rb.len(), 2);
        assert!(!rb.is_empty());
        assert_eq!(rb.push('f'), Ok(()));
        assert_eq!(rb.len(), 3);
        assert!(!rb.is_empty());
        assert_eq!(rb.push('g'), Ok(()));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.push('h'), Ok(()));
        assert_eq!(rb.len(), 5);
        assert_eq!(rb.push('i'), Err(RollingCharBufferError::BufferFullError));
        assert!(!rb.is_empty());
        assert_eq!(rb.read(), Ok('d'));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.push('i'), Ok(()));
        assert_eq!(rb.len(), 5);
        assert_eq!(rb.read(), Ok('e'));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.read(), Ok('f'));
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.read(), Ok('g'));
        assert_eq!(rb.len(), 2);
        assert_eq!(rb.read(), Ok('h'));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.read(), Ok('i'));
        assert_eq!(rb.len(), 0);
        assert_eq!(rb.read(), Err(RollingCharBufferError::BufferEmptyError));
        assert!(rb.is_empty());
        assert_eq!(rb.push('j'), Ok(()));
        assert_eq!(rb.len(), 1);
        assert!(!rb.is_empty());
    }

    #[test]
    fn test_buffer_extends() {
        let mut rb = RollingCharBuffer::<5>::new();

        assert_eq!(rb.push('a'), Ok(()));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.extend(&['b', 'c', 'd']), Ok(1));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.read(), Ok('a'));
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.read(), Ok('b'));
        assert_eq!(rb.len(), 2);
        assert_eq!(rb.read(), Ok('c'));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.extend(&['e', 'f', 'g', 'h']), Ok(0));
        assert_eq!(rb.len(), 5);
        assert_eq!(
            rb.extend(&['i', 'j', 'k', 'l']),
            Err(RollingCharBufferError::BufferFullError)
        );
        assert_eq!(rb.read(), Ok('d'));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.read(), Ok('e'));
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.read(), Ok('f'));
        assert_eq!(rb.len(), 2);
        assert_eq!(
            rb.extend(&['i', 'j', 'k', 'l']),
            Err(RollingCharBufferError::BufferFullError)
        );
        assert_eq!(rb.read(), Ok('g'));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.read(), Ok('h'));
        assert_eq!(rb.len(), 0);
        assert!(rb.is_empty());
    }

    #[test]
    fn test_buffer_prepends_one_char() {
        let mut rb = RollingCharBuffer::<5>::new();

        assert_eq!(rb.prefix('t'), Ok(()));
        assert_eq!(rb.len(), 1);
        assert!(!rb.is_empty());
        assert_eq!(rb.read(), Ok('t'));
        assert_eq!(rb.read(), Err(RollingCharBufferError::BufferEmptyError));
        assert!(rb.is_empty());
    }

    #[test]
    fn test_buffer_adds_prepends_one_char() {
        let mut rb = RollingCharBuffer::<5>::new();

        assert_eq!(rb.push('t'), Ok(()));
        assert_eq!(rb.prefix('i'), Ok(()));
        assert_eq!(rb.len(), 2);
        assert!(!rb.is_empty());
        assert_eq!(rb.read(), Ok('i'));
        assert_eq!(rb.read(), Ok('t'));
        assert_eq!(rb.read(), Err(RollingCharBufferError::BufferEmptyError));
        assert!(rb.is_empty());
    }

    #[test]
    fn test_buffer_extends_and_prepend() {
        let mut rb = RollingCharBuffer::<8>::new();

        assert_eq!(rb.push('a'), Ok(()));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.extend(&['b', 'c', 'd']), Ok(4));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.read(), Ok('a'));
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.read(), Ok('b'));
        assert_eq!(rb.len(), 2);
        assert_eq!(rb.read(), Ok('c'));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.extend(&['e', 'f', 'g', 'h']), Ok(3));
        assert_eq!(rb.prepend(&['a', 'b', 'c']), Ok(0));
        assert_eq!(rb.len(), 8);
        assert_eq!(
            rb.extend(&['i', 'j', 'k', 'l']),
            Err(RollingCharBufferError::BufferFullError)
        );
        assert_eq!(rb.read(), Ok('a'));
        assert_eq!(rb.len(), 7);
        assert_eq!(rb.read(), Ok('b'));
        assert_eq!(rb.len(), 6);
        assert_eq!(rb.read(), Ok('c'));
        assert_eq!(rb.len(), 5);
        assert_eq!(rb.extend(&['i', 'j']), Ok(1));
        assert_eq!(rb.prepend(&['c']), Ok(0));
        assert_eq!(rb.read(), Ok('c'));
        assert_eq!(rb.len(), 7);
        assert_eq!(rb.read(), Ok('d'));
        assert_eq!(rb.len(), 6);
        assert_eq!(rb.read(), Ok('e'));
        assert_eq!(rb.len(), 5);
        assert_eq!(rb.read(), Ok('f'));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.read(), Ok('g'));
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.read(), Ok('h'));
        assert_eq!(rb.len(), 2);
        assert_eq!(rb.read(), Ok('i'));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.read(), Ok('j'));
        assert_eq!(rb.len(), 0);
        assert!(rb.is_empty());
    }
}
