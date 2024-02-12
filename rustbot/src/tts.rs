use crate::{buffer, prelude::*};
use bytes::Bytes;
use playht_rs::api::{self, stream::TTSStreamReq, tts::Quality};
use tokio::{self, sync::mpsc::Receiver, sync::watch};

#[derive(Debug, Clone)]
pub struct Config {
    pub voice_id: Option<Str