use bytes::{BufMut, Bytes, BytesMut};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BufferFullError {
    pub bytes_written: usize,
}

impl fmt::Display for Buffer