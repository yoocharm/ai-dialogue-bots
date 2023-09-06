use bytes::{BufMut, Bytes, BytesMut};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BufferFullError {
    pub bytes_written: usize,
}

impl fmt::Display for BufferFullError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "buffer is full, {} bytes written", self.bytes_written)
    }
}

impl Error for BufferFullError {}

pub struct Buffer {
    buffer: BytesMut,
    max_size: usize,
}

impl Buffer {
    pub fn new(max_size: usize) -> Self {
        Buffer {
            buffer: BytesMut::with_capacity(max_size),
            max_size,
        }
    }

    pub fn write