# SSI FastConnect Data Rust Client

This repository now contains a library-first Rust replacement for the SSI Node.js and Python
FastConnect Data samples. The original `fc-data.node/` and `fc-data.py/` directories remain as
protocol references. The Rust workspace separates the primary library at `crates/fc-data/`
from the thin optional binary at `crates/fc-data-cli/`.

The Rust client supports:

- typed configuration and secret handling;
- all eight SSI Market Data REST v2 request models;
- authenticated JSON execution through `MarketDataClient`;
- bounded realtime subscriptions through `LegacyStreamClient`;
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
    api::{ApiRequest, MarketDataClient, PageQuery, SecuritiesQuery},
    config::Settings,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MarketDataClient::new(Settings::load()?)?;
    let page = PageQuery::new(1, 10)?;
    let query = SecuritiesQuery::new(Some("HOSE".to_owned()), page)?;
    let request = ApiRequest::Securities(query);
    let response = client.execute(&request).await?;
    serde_json::to_writer_pretty(std::io::stdout(), &response)?;
    Ok(())
}
```

For realtime data, construct `LegacyStreamClient` from the same authenticated
`MarketDataClient` and pass validated `StreamOptions`.

Library request structs have private fields. Use their `new` or `parse` functions so invalid
page sizes, dates, required symbols, market codes, order values, and resolutions are rejected
before authentication or network I/O.

## Credentials

The local `.opencode/opencode.jsonc` enables the Bitwarden MCP server. Credentials were loaded
from the Bitwarden item named `SSI FCDATA` into the ignored root `.env` file.

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
cargo test --workspace --all-targets
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
- `daily-index`
- `daily-stock-price`

Dates use SSI's `DD/MM/YYYY` format. Pagination is validated before any network request.

## Protocol notes

SSI streaming uses the legacy `/negotiate` -> WebSocket `/connect` -> HTTP `/start` sequence,
not the ASP.NET Core SignalR handshake implemented by most modern SignalR crates. The Rust
client therefore uses `reqwest` and `tokio-tungstenite` directly with hub
`fcmarketdatav2hub` and method `SwitchChannels`.
