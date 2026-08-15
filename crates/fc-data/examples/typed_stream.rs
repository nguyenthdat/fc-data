#![doc = "Typed SSI stream channel and payload example."]

use std::{error::Error, io, io::Write as _, time::Duration};

use serde_json::json;
use ssi_fc_data::{
    api::MarketDataClient,
    config::Settings,
    stream::{Channel, ChannelSelector, StreamClient, StreamMessage, StreamOptions},
};

type ExampleResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> ExampleResult {
    let mut arguments = std::env::args().skip(1);
    let first = arguments.next();
    let render_only = first.as_deref() == Some("--render");
    let family = if render_only {
        "quote"
    } else {
        first.as_deref().unwrap_or("quote")
    };
    let default_subject = if family == "index" { "VN30" } else { "SSI" };
    let subject = arguments
        .next()
        .unwrap_or_else(|| default_subject.to_owned());
    let selector = if subject == "ALL" {
        ChannelSelector::all()
    } else {
        ChannelSelector::symbols([subject])?
    };
    let channel = match family {
        "status" => Channel::securities_status(&selector),
        "quote" => Channel::quote(&selector),
        "trade" => Channel::trade(&selector),
        "room" => Channel::foreign_room(&selector),
        "index" => Channel::index(&selector),
        "bar" => Channel::bar(&selector),
        other => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unsupported stream family {other}"),
            )
            .into());
        }
    };
    if render_only {
        writeln!(std::io::stdout(), "{channel}")?;
        return Ok(());
    }

    let client = MarketDataClient::new(Settings::load()?)?;
    let options = StreamOptions::from_channel(&channel, 1, Duration::from_secs(20))?;
    let message = StreamClient::new(&client)
        .collect_typed(&options)
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "empty SSI stream"))?;
    let (data_type, subject) = describe(&message);

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    serde_json::to_writer_pretty(
        &mut lock,
        &json!({"channel": channel.as_str(), "dataType": data_type, "subject": subject}),
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
