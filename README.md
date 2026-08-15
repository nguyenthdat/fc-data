# SSI FastConnect Data Rust Client

This repository contains a library-first Rust client for SSI FastConnect Data. The workspace
separates the primary library at `crates/fc-data/` from the thin optional binary at
`crates/fc-data-cli/`.

The Rust client supports:

- typed configuration and secret handling;
- typed requests and capture-backed responses for all eight REST APIs documented by SSI v2.2;
- typed compatibility for the official .NET client's `IntradaybyTick` REST operation;
- an additional raw `BackTest` request retained for compatibility;
- typed channels and payloads for `F`, `X-QUOTE`, `X-TRADE`, `R`, `MI`, and `B` streams;
- raw JSON and raw channel escape hatches for forward compatibility;
- bounded and persistent realtime subscriptions through `StreamClient`, with opt-in reconnect and
  resubscribe support;
- an optional JSON CLI in its own workspace crate.

```text
crates/
├── fc-data/      # package: ssi-fc-data, public library
└── fc-data-cli/  # package: fc-data-cli, binary: fc-data
```

## Library usage

Build the library independently of the companion binary:

```bash
cargo build -p ssi-fc-data --lib
```

Execute a typed securities request:

```rust
use ssi_fc_data::{
    api::{MarketDataClient, PageQuery, SecuritiesQuery, SecuritiesResponse},
    config::Settings,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MarketDataClient::new(Settings::load()?)?;
    let page = PageQuery::new(1, 10)?;
    let query = SecuritiesQuery::new(Some("HOSE".to_owned()), page)?;
    let response: SecuritiesResponse = client.execute_typed(&query).await?;
    serde_json::to_writer_pretty(std::io::stdout(), &response)?;
    Ok(())
}
```

Every concrete typed REST request implements `RestRequest`, which fixes its response payload at
compile time. `MarketDataClient::execute` and `ApiRequest` remain available when a caller needs
the untyped SSI JSON envelope.

For realtime data, construct a `Channel` from a validated `ChannelSelector`, then use
`collect_typed`, `subscribe_typed`, `recv_typed`, and `switch_typed`. Unknown stream data types
are preserved by `StreamMessage::Unknown` instead of being discarded. Raw strings and JSON
remain available through the explicitly named raw methods.

Run a live typed quote decode with:

```bash
cargo run -p ssi-fc-data --example typed_stream
```

Run a persistent same-session switch with the raw compatibility example:

```bash
cargo run -p ssi-fc-data --example live_switch -- MI:VN30 X-QUOTE:SSI
```

Run an opt-in resilient typed subscription that restores its last channel after transport loss:

```bash
cargo run -p ssi-fc-data --example resilient_stream
```

Library request structs have private fields. Use their `new` or `parse` functions so invalid
endpoint-specific page sizes, exact `DD/MM/YYYY` dates, date ranges, required symbols, market
codes, exchange codes, order values, and resolutions are rejected before authentication or
network I/O. Intraday dates are independently optional and are omitted from the query when not
provided.

## Credentials

Copy `.env.example` to the ignored root `.env` file and populate the SSI credentials.

Required runtime variables:

```dotenv
SSI_FCDATA_CONSUMER_ID=
SSI_FCDATA_CONSUMER_SECRET=
SSI_FCDATA_API_URL=https://fc-data.ssi.com.vn/
SSI_FCDATA_STREAM_URL=https://fc-datahub.ssi.com.vn/
```

The Bitwarden `PublicKey` and `PrivateKey` fields are also retained locally as
`SSI_FCDATA_PUBLIC_KEY` and `SSI_FCDATA_PRIVATE_KEY`, but FCData market queries do not use them.
Never commit `.env` or print its values.

## Build and verify

```bash
cargo build -p ssi-fc-data --lib --release
cargo build -p fc-data-cli --bin fc-data --release
cargo nextest run --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Companion binary

The binary lives entirely under `crates/fc-data-cli/`; no CLI types are exported by the library.
Show its complete command surface:

```bash
cargo run -p fc-data-cli --bin fc-data -- --help
```

Verify credentials without exposing the eight-hour bearer token:

```bash
cargo run -p fc-data-cli --bin fc-data -- auth
```

Query securities:

```bash
cargo run -p fc-data-cli --bin fc-data -- \
  securities --market HOSE --page-index 1 --page-size 10
```

Query historical data:

```bash
cargo run -p fc-data-cli --bin fc-data -- daily-ohlc \
  --symbol SSI \
  --from-date 13/08/2026 \
  --to-date 14/08/2026 \
  --page-size 10
```

Query the official .NET client's intraday-by-tick operation:

```bash
cargo run -p fc-data-cli --bin fc-data -- intraday-by-tick \
  --symbol SSI \
  --from-date 14/08/2026 \
  --to-date 14/08/2026 \
  --page-size 10
```

The official .NET v2.0.0 source exposes this operation, but SSI's production endpoint currently
returns HTTP 404 for its declared `api/v2/Market/IntradaybyTick` path.

Query the SSI `BackTest` endpoint:

```bash
cargo run -p fc-data-cli --bin fc-data -- backtest \
  --selected-date 14/08/2026 \
  --symbol SSI
```

The request model is retained for API parity, although the live SSI endpoint currently replies
with `"Not support"`.

Collect one realtime quote broadcast, bounded to 20 seconds:

```bash
cargo run -p fc-data-cli --bin fc-data -- stream \
  --channel X-QUOTE:ALL \
  --max-messages 1 \
  --timeout-seconds 20
```

Available REST subcommands:

- `securities`
- `securities-details`
- `index-components`
- `index-list`
- `daily-ohlc`
- `intraday-ohlc`
- `intraday-by-tick`
- `daily-index`
- `daily-stock-price`
- `backtest`

Dates use SSI's exact `DD/MM/YYYY` format. Pagination and endpoint-specific request constraints
are validated before any network request.

## Protocol notes

SSI streaming uses the `SignalR` 1.3 `/negotiate` -> WebSocket `/connect` -> HTTP `/start` sequence,
not the ASP.NET Core SignalR handshake implemented by most modern SignalR crates. The Rust
client therefore uses `reqwest` and `tokio-tungstenite` directly with hub
`fcmarketdatav2hub` and method `SwitchChannels`. Existing subscriptions do not silently
reconnect. Callers can opt into `ResilientSubscription`, whose default policy matches the
official .NET client by retrying once after three seconds and restoring the latest channel.

Public enums are `#[non_exhaustive]`; downstream matches must include a wildcard arm so SSI
protocol and validation cases can evolve without a breaking release.

## License

Licensed under the [MIT License](LICENSE).
