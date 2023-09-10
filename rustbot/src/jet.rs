
use crate::prelude::*;
use async_nats::jetstream::{
    self,
    consumer::{pull, Consumer},
    stream,
};
use bytes::{Bytes, BytesMut};
use tokio::{
    self,
    sync::mpsc::{Receiver, Sender},
    sync::watch,
};
use tokio_stream::StreamExt;

#[derive(Clone, Debug)]
pub struct Config {
    pub nats_url: String,
    pub durable_name: String,
    pub stream_name: String,
    pub pub_subject: String,
    pub sub_subject: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            nats_url: std::env::var("NATS_URL").unwrap_or_else(|_| NATS_DEFAULT_URL.to_string()),
            durable_name: BOT_NAME.to_string(),
            stream_name: STREAM_NAME.to_string(),
            pub_subject: BOT_PUB_SUBJECT.to_string(),
            sub_subject: BOT_SUB_SUBJECT.to_string(),
        }
    }
}

pub struct Stream {
    pub writer: Writer,
    pub reader: Reader,
}

impl Stream {
    pub async fn new(c: Config) -> Result<Self> {
        let client = async_nats::connect(c.nats_url).await?;
        let js = jetstream::new(client);

        let stream = js
            .get_or_create_stream(stream::Config {