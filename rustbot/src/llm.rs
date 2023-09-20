use crate::{history, prelude::*};
use bytes::Bytes;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use tokio::{
    self,
    sync::mpsc::{Receiver, Sender},
    sync::watch,
    task::JoinHandle,
};
use tokio_stream::StreamExt;

#[derive(Clone, Debug)]
pub struct Config {
    pub hist_size: usize,
    pub model_name: String,
    pub seed_prompt: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            hist_size: HISTORY_SIZE,
            model_name: DE