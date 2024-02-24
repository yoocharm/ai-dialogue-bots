use crate::{buffer, prelude::*};
use bytes::Bytes;
use playht_rs::api::{self, stream::TTSStreamReq, tts::Quality};
use tokio::{self, sync::mpsc::Receiver, sync::watch};

#[derive(Debug, Clone)]
pub struct Config {
    pub voice_id: Option<String>,
    pub quality: Option<Quality>,
    pub speed: Option<f32>,
    pub sample_rate: Option<i32>,
    pub buf_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            voice_id: Some(DEFAULT_VOICE_ID.to_string()),
            quality: Some(Quality::Low),
            speed: Some(1.0),
            sample_rate: Some(24000),
            buf_size: MAX_TTS_BUFFER_SIZE,
        }
    }
}

pub struct TTS {
    client: api::Client,
    config: Config,
}

impl TTS {
    pub fn new(c: Config) -> TTS {
        TTS {
            client: api::Client::new(),
            config: c,
        }
    }

    pub async fn stream<