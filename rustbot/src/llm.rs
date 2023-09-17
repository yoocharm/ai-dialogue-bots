use crate::{history, prelude::*};
use bytes::Bytes;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use tokio::{
    self,
    sync::mpsc::{Receiver, Sender},
    sync::watch,
    task::JoinHandle,
};
use tokio_stream::Str