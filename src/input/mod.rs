pub mod inputs;
pub const BUFFER_SIZE: usize = 10240;

pub trait Input {
    fn next(&mut self) -> Option<char>;
}
