#![doc = "Live same-session channel-switch example for SSI streaming."]

use std::{error::Error, io, io::Write as _, time::Duration};

use serde_json::{Value, json};
use ssi_fc_data::{
    api::MarketDataClient,
    config::Settings,
    stream::{StreamClient, Subscription},
};

type ExampleResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> ExampleResult {
    let mut arguments = std::env::args().skip(1);
    let first_channel = arguments
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing first channel"))?;
    let second_channel = arguments
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing second channel"))?;
    let timeout = Duration::from_secs(20);
    let client = MarketDataClient::new(Settings::load()?)?;
    let stream = StreamClient::new(&client);
    let mut subscription = stream.subscribe(&first_channel, timeout).await?;
    let first = receive(&mut subscription, timeout).await?;
    subscription.switch_channel(&second_channel).await?;
    let second = receive(&mut subscription, timeout).await?;
    subscription.close().await?;

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    serde_json::to_writer_pretty(
        &mut lock,
        &json!({
            "firstChannel": first_channel,
            "first": first,
            "secondChannel": second_channel,
            "second": second
        }),
    )?;
    writeln!(lock)?;
    Ok(())
}

async fn receive(subscription: &mut Subscription, timeout: Duration) -> ExampleResult<Value> {
    tokio::time::timeout(timeout, subscription.recv())
        .await??
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "SSI closed the stream").into())
}
