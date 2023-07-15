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
    println!("launching audio player");
    let mut audio_data = BytesMut::new();
    // TODO: make this a cli switch as this value has been picked rather arbitrarily
    let interval_duration = Duration::from_millis(AUDIO_INTERVAL);
    let mut interval = time::interval(interval_duration);
    let mut last_play_time = Instant::now();
    let mut has_played_audio = false;

    loop {
        tokio::select! {
            _ = done.changed() => {
                if *done.borrow() {
                    break;
                }
            }
            result = audio_rd.read_buf(&mut audio_data) => {
                if let Ok(chunk) = result {
                    if chunk == 0 {
                        break;
                    }
               