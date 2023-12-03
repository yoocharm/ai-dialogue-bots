use bytes::Bytes;
use clap::Parser;
use prelude::*;
use rodio::{OutputStream, Sink};
use tokio::{
    self, io,
    sync::{mpsc, watch},
};

mod audio;
mod buffer;
mod cli;
mod histor