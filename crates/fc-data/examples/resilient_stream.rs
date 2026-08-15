#![doc = "Opt-in resilient typed SSI stream example."]

use std::{error::Error, io::Write as _, time::Duration};

use serde_json::json;
use ssi_fc_data::{
    api::MarketDataClient,
    config::Settings,
    stream::{Channel, ChannelSelector, ReconnectOptions, StreamClient, StreamMessage},
};

type ExampleResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> ExampleResult {
    let subject = std::env::args().nth(1).unwrap_or_else(|| "SSI".to_owned());
    let selector = if subject == "ALL" {
        ChannelSelector::all()
    } else {
        ChannelSelector::symbols([subject])?
    };
    let channel = Channel::quote(&selector);
    let options = ReconnectOptions::new(Duration::from_secs(20))?;
    let client = MarketDataClient::new(Settings::load()?)?;
    let mut subscription = StreamClient::new(&client)
        .subscribe_resilient_typed(&channel, options)
        .await?;
    let message =
        tokio::time::timeout(Duration::from_secs(30), subscription.recv_typed()).await??;
    subscription.close().await?;
    let (data_type, subject) = describe(&message);

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    serde_json::to_writer_pretty(
        &mut lock,
        &json!({
            "channel": channel.as_str(),
            "dataType": data_type,
            "subject": subject,
            "maxReconnects": options.policy().max_retries(),
            "reconnectDelaySeconds": options.policy().delay().as_secs()
        }),
    )?;
    writeln!(lock)?;
    Ok(())
}

fn describe(message: &StreamMessage) -> (&str, &str) {
    match message {
        StreamMessage::SecuritiesStatus(value) => ("F", &value.symbol),
        StreamMessage::Quote(value) => ("X-QUOTE", &value.symbol),
        StreamMessage::Trade(value) => ("X-TRADE", &value.symbol),
        StreamMessage::ForeignRoom(value) => ("R", &value.symbol),
        StreamMessage::Index(value) => ("MI", &value.index_id),
        StreamMessage::Bar(value) => ("B", &value.symbol),
        StreamMessage::Unknown { data_type, .. } => (data_type, "unknown"),
        _ => ("UNMODELED", "unknown"),
    }
}
