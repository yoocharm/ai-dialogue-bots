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
            model_name: DEFAULT_MODEL_NAME.to_string(),
            seed_prompt: None,
        }
    }
}

pub struct LLM {
    client: Ollama,
    model_name: String,
    hist_size: usize,
    seed_prompt: Option<String>,
}

impl LLM {
    pub fn new(c: Config) -> Self {
        let ollama = Ollama::default();
        LLM {
            client: ollama,
            model_name: c.model_name,
            hist_size: c.hist_size,
            seed_prompt: c.seed_prompt,
        }
    }

    pub async fn stream(
        self,