use crate::prelude::*;
use bytes::BytesMut;
use rodio::{Decoder, Sink};
use std::io::Cursor;
use tokio::{
    self,
    io::{self, AsyncReadExt},
    sync::watch,
    time::{self, Duration, Instant},
};

pub async fn play(
    mut audio_rd: io::DuplexStream,
    sink: Sink,
    audio_done: watch::Sender<bool>,
    mut done: watch::Receiver<bool>,
) -> Result<()> {
 