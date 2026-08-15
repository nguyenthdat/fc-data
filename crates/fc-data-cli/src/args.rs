//! Strongly typed command-line arguments.

use clap::{Args, Parser, Subcommand, ValueEnum};

/// SSI `FastConnect` Data command-line client.
#[derive(Debug, Parser)]
#[command(
    name = "fc-data",
    version,
    about = "Query and stream SSI FastConnect Data from Rust",
    arg_required_else_help = true,
    propagate_version = true
)]
pub(super) struct Cli {
    /// Operation to perform.
    #[command(subcommand)]
    pub(super) command: Command,
}

/// Supported SSI operations.
#[derive(Debug, Subcommand)]
pub(super) enum Command {
    /// Verify credentials without printing the access token.
    Auth,
    /// List securities.
    Securities(SecuritiesArgs),
    /// Get securities details.
    SecuritiesDetails(SecuritiesDetailsArgs),
    /// Get index components.
    IndexComponents(IndexComponentsArgs),
    /// List indexes.
    IndexList(IndexListArgs),
    /// Get daily OHLC data.
    DailyOhlc(DailyOhlcArgs),
    /// Get intraday OHLC data.
    IntradayOhlc(IntradayOhlcArgs),
    /// Get unaggregated intraday tick data.
    IntradayByTick(IntradayByTickArgs),
    /// Get daily index data.
    DailyIndex(DailyIndexArgs),
    /// Get daily stock prices.
    DailyStockPrice(DailyStockPriceArgs),
    /// Query historical `BackTest` data.
    Backtest(BacktestArgs),
    /// Collect a bounded number of realtime streaming messages.
    Stream(StreamArgs),
}

/// SSI market filter.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub(super) enum Market {
    /// Ho Chi Minh Stock Exchange.
    Hose,
    /// Hanoi Stock Exchange.
    Hnx,
    /// Unlisted Public Company Market.
    Upcom,
    /// Derivatives market.
    Der,
    /// Bond market.
    Bond,
}

/// Market filter accepted by the securities endpoint.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub(super) enum SecuritiesMarket {
    /// Ho Chi Minh Stock Exchange.
    Hose,
    /// Hanoi Stock Exchange.
    Hnx,
    /// Unlisted Public Company Market.
    Upcom,
    /// Derivatives market.
    Der,
}

/// Exchange filter accepted by the index-list endpoint.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub(super) enum Exchange {
    /// Ho Chi Minh Stock Exchange.
    Hose,
    /// Hanoi Stock Exchange.
    Hnx,
}

/// Sort direction accepted by SSI.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub(super) enum Order {
    /// Ascending order.
    Asc,
    /// Descending order.
    Desc,
}

/// Shared pagination flags.
#[derive(Debug, Clone, Copy, Args)]
pub(super) struct PageArgs {
    /// One-based page number from 1 to 10.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=10))]
    pub(super) page_index: u8,
    /// SSI-supported page size.
    #[arg(long, default_value_t = 100, value_parser = parse_page_size)]
    pub(super) page_size: u16,
}

/// Pagination flags for securities endpoints.
#[derive(Debug, Clone, Copy, Args)]
pub(super) struct SecuritiesPageArgs {
    /// One-based page number from 1 to 10.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=10))]
    pub(super) page_index: u8,
    /// SSI-supported securities page size.
    #[arg(long, default_value_t = 100, value_parser = parse_securities_page_size)]
    pub(super) page_size: u16,
}

/// Securities list flags.
#[derive(Debug, Args)]
pub(super) struct SecuritiesArgs {
    /// Optional market filter.
    #[arg(long, value_enum, ignore_case = true)]
    pub(super) market: Option<SecuritiesMarket>,
    /// Pagination flags.
    #[command(flatten)]
    pub(super) page: SecuritiesPageArgs,
}

/// Securities details flags.
#[derive(Debug, Args)]
pub(super) struct SecuritiesDetailsArgs {
    /// Optional market filter.
    #[arg(long, value_enum, ignore_case = true)]
    pub(super) market: Option<Market>,
    /// Optional instrument symbol.
    #[arg(long)]
    pub(super) symbol: Option<String>,
    /// Pagination flags.
    #[command(flatten)]
    pub(super) page: SecuritiesPageArgs,
}

/// Index components flags.
#[derive(Debug, Args)]
pub(super) struct IndexComponentsArgs {
    /// SSI index code such as VN30.
    #[arg(long)]
    pub(super) index_code: String,
    /// Pagination flags.
    #[command(flatten)]
    pub(super) page: PageArgs,
}

/// Index list flags.
#[derive(Debug, Args)]
pub(super) struct IndexListArgs {
    /// Optional exchange filter.
    #[arg(long, value_enum, ignore_case = true)]
    pub(super) exchange: Option<Exchange>,
    /// Pagination flags.
    #[command(flatten)]
    pub(super) page: PageArgs,
}

/// Daily OHLC flags.
#[derive(Debug, Args)]
pub(super) struct DailyOhlcArgs {
    /// Optional stock, index, or derivative symbol.
    #[arg(long)]
    pub(super) symbol: Option<String>,
    /// Start date in DD/MM/YYYY form.
    #[arg(long)]
    pub(super) from_date: String,
    /// End date in DD/MM/YYYY form.
    #[arg(long)]
    pub(super) to_date: String,
    /// Return records in ascending order.
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub(super) ascending: bool,
    /// Pagination flags.
    #[command(flatten)]
    pub(super) page: PageArgs,
}

/// Intraday OHLC flags.
#[derive(Debug, Args)]
pub(super) struct IntradayOhlcArgs {
    /// Stock, derivative, or covered-warrant symbol.
    #[arg(long)]
    pub(super) symbol: String,
    /// Optional start date in DD/MM/YYYY form.
    #[arg(long)]
    pub(super) from_date: Option<String>,
    /// Optional end date in DD/MM/YYYY form.
    #[arg(long)]
    pub(super) to_date: Option<String>,
    /// Return records in ascending order.
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub(super) ascending: bool,
    /// Aggregation resolution in minutes.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u16).range(1..))]
    pub(super) resolution: u16,
    /// Pagination flags.
    #[command(flatten)]
    pub(super) page: PageArgs,
}

/// Intraday-by-tick flags.
#[derive(Debug, Args)]
pub(super) struct IntradayByTickArgs {
    /// Security symbol.
    #[arg(long)]
    pub(super) symbol: String,
    /// Start date in DD/MM/YYYY form.
    #[arg(long)]
    pub(super) from_date: String,
    /// End date in DD/MM/YYYY form.
    #[arg(long)]
    pub(super) to_date: String,
    /// One-based page number from 1 to 10.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=10))]
    pub(super) page_index: u8,
    /// SSI-supported page size.
    #[arg(long, default_value_t = 10, value_parser = parse_page_size)]
    pub(super) page_size: u16,
}

/// Daily index flags.
#[derive(Debug, Args)]
pub(super) struct DailyIndexArgs {
    /// Caller correlation ID.
    #[arg(long, default_value = "rust-client")]
    pub(super) request_id: String,
    /// SSI index identifier such as VN30.
    #[arg(long)]
    pub(super) index_id: String,
    /// Start date in DD/MM/YYYY form.
    #[arg(long)]
    pub(super) from_date: String,
    /// End date in DD/MM/YYYY form.
    #[arg(long)]
    pub(super) to_date: String,
    /// SSI ordering field.
    #[arg(long, default_value = "TradingDate")]
    pub(super) order_by: String,
    /// SSI ordering direction.
    #[arg(long, value_enum, ignore_case = true, default_value_t = Order::Desc)]
    pub(super) order: Order,
    /// Return records in ascending order.
    #[arg(long, default_value_t = false, action = clap::ArgAction::Set)]
    pub(super) ascending: bool,
    /// Pagination flags.
    #[command(flatten)]
    pub(super) page: PageArgs,
}

/// Daily stock price flags.
#[derive(Debug, Args)]
pub(super) struct DailyStockPriceArgs {
    /// Optional instrument symbol.
    #[arg(long)]
    pub(super) symbol: Option<String>,
    /// Start date in DD/MM/YYYY form.
    #[arg(long)]
    pub(super) from_date: String,
    /// End date in DD/MM/YYYY form.
    #[arg(long)]
    pub(super) to_date: String,
    /// Optional market filter.
    #[arg(long, value_enum, ignore_case = true)]
    pub(super) market: Option<Market>,
    /// One-based page number from 1 to 10.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=10))]
    pub(super) page_index: u8,
    /// SSI-supported daily stock page size.
    #[arg(long, default_value_t = 100, value_parser = parse_stock_page_size)]
    pub(super) page_size: u16,
}

/// `BackTest` flags.
#[derive(Debug, Args)]
pub(super) struct BacktestArgs {
    /// Selected trading date passed through to SSI.
    #[arg(long)]
    pub(super) selected_date: String,
    /// Instrument symbol.
    #[arg(long)]
    pub(super) symbol: String,
}

/// Realtime stream flags.
#[derive(Debug, Args)]
pub(super) struct StreamArgs {
    /// SSI channel such as X-QUOTE:ALL or MI:VN30.
    #[arg(long)]
    pub(super) channel: String,
    /// Number of matching broadcasts to collect.
    #[arg(long, default_value_t = 1, value_parser = parse_positive_usize)]
    pub(super) max_messages: usize,
    /// Maximum wait for matching broadcasts.
    #[arg(long, default_value_t = 15, value_parser = parse_positive_u64)]
    pub(super) timeout_seconds: u64,
}

impl Market {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Hose => "HOSE",
            Self::Hnx => "HNX",
            Self::Upcom => "UPCOM",
            Self::Der => "DER",
            Self::Bond => "BOND",
        }
    }
}

impl SecuritiesMarket {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Hose => "HOSE",
            Self::Hnx => "HNX",
            Self::Upcom => "UPCOM",
            Self::Der => "DER",
        }
    }
}

impl Exchange {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Hose => "HOSE",
            Self::Hnx => "HNX",
        }
    }
}

impl Order {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}

fn parse_page_size(value: &str) -> Result<u16, String> {
    parse_size(value, &[10, 20, 50, 100, 500, 1000])
}

fn parse_securities_page_size(value: &str) -> Result<u16, String> {
    parse_size(value, &[10, 20, 50, 100, 1000])
}

fn parse_stock_page_size(value: &str) -> Result<u16, String> {
    parse_size(value, &[10, 20, 50, 100])
}

fn parse_size(value: &str, allowed: &[u16]) -> Result<u16, String> {
    let size = value
        .parse::<u16>()
        .map_err(|error| format!("page size must be an integer: {error}"))?;
    if allowed.contains(&size) {
        Ok(size)
    } else {
        Err(format!("page size must be one of {allowed:?}"))
    }
}

fn parse_positive_usize(value: &str) -> Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|error| format!("value must be a positive integer: {error}"))?;
    if parsed == 0 {
        Err("value must be greater than zero".to_owned())
    } else {
        Ok(parsed)
    }
}

fn parse_positive_u64(value: &str) -> Result<u64, String> {
    let parsed = value
        .parse::<u64>()
        .map_err(|error| format!("value must be a positive integer: {error}"))?;
    if parsed == 0 {
        Err("value must be greater than zero".to_owned())
    } else {
        Ok(parsed)
    }
}
