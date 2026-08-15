//! CLI command execution and JSON output.

use std::{io::Write as _, time::Duration};

use serde_json::Value;
use thiserror::Error;

use super::args::{
    BacktestArgs, Cli, Command, DailyIndexArgs, DailyOhlcArgs, DailyStockPriceArgs,
    IndexComponentsArgs, IndexListArgs, IntradayOhlcArgs, PageArgs, SecuritiesArgs,
    SecuritiesDetailsArgs, SecuritiesPageArgs, StreamArgs,
};
use ssi_fc_data::{
    api::{
        ApiRequest, BacktestQuery, ClientError, DailyIndexInput, DailyIndexQuery, DailyOhlcInput,
        DailyOhlcQuery, DailyStockPriceInput, DailyStockPriceQuery, IndexComponentsQuery,
        IndexListQuery, IntradayOhlcInput, IntradayOhlcQuery, MarketDataClient, PageQuery,
        RestRequest, SecuritiesDetailsQuery, SecuritiesQuery, ValidationError,
    },
    config::{ConfigError, Settings},
    stream::{StreamClient, StreamError, StreamOptions},
};

/// CLI execution failure.
#[derive(Debug, Error)]
pub(super) enum RunError {
    /// Runtime configuration was invalid.
    #[error(transparent)]
    Config(#[from] ConfigError),
    /// An SSI REST request failed.
    #[error(transparent)]
    Client(#[from] ClientError),
    /// Library request validation rejected CLI input.
    #[error(transparent)]
    Validation(#[from] ValidationError),
    /// An SSI streaming request failed.
    #[error(transparent)]
    Stream(#[from] StreamError),
    /// JSON output serialization failed.
    #[error("failed to serialize JSON output: {0}")]
    Json(#[from] serde_json::Error),
    /// Writing JSON to stdout failed.
    #[error("failed to write command output: {0}")]
    Output(#[from] std::io::Error),
}

/// Executes a parsed command without exposing credentials or access tokens.
pub(super) async fn run(cli: Cli) -> Result<(), RunError> {
    let client = MarketDataClient::new(Settings::load()?)?;
    let output = match cli.command {
        Command::Auth => {
            client.authenticate().await?;
            serde_json::json!({"authenticated": true})
        }
        Command::Securities(args) => {
            execute_typed(&client, SecuritiesQuery::try_from(args)?).await?
        }
        Command::SecuritiesDetails(args) => {
            execute_typed(&client, SecuritiesDetailsQuery::try_from(args)?).await?
        }
        Command::IndexComponents(args) => {
            execute_typed(&client, IndexComponentsQuery::try_from(args)?).await?
        }
        Command::IndexList(args) => execute_typed(&client, IndexListQuery::try_from(args)?).await?,
        Command::DailyOhlc(args) => execute_typed(&client, DailyOhlcQuery::try_from(args)?).await?,
        Command::IntradayOhlc(args) => {
            execute_typed(&client, IntradayOhlcQuery::try_from(args)?).await?
        }
        Command::DailyIndex(args) => {
            execute_typed(&client, DailyIndexQuery::try_from(args)?).await?
        }
        Command::DailyStockPrice(args) => {
            execute_typed(&client, DailyStockPriceQuery::try_from(args)?).await?
        }
        Command::Backtest(args) => execute_raw(&client, args.try_into()?).await?,
        Command::Stream(args) => execute_stream(&client, args).await?,
    };
    write_json(&output)
}

async fn execute_raw(client: &MarketDataClient, request: ApiRequest) -> Result<Value, ClientError> {
    client.execute(&request).await
}

async fn execute_typed<R>(client: &MarketDataClient, request: R) -> Result<Value, RunError>
where
    R: RestRequest,
{
    let response = client.execute_typed(&request).await?;
    Ok(serde_json::to_value(response)?)
}

async fn execute_stream(client: &MarketDataClient, args: StreamArgs) -> Result<Value, StreamError> {
    let options = StreamOptions::new(
        args.channel,
        args.max_messages,
        Duration::from_secs(args.timeout_seconds),
    )?;
    let payloads = StreamClient::new(client).collect(&options).await?;
    Ok(Value::Array(payloads))
}

fn write_json(value: &Value) -> Result<(), RunError> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    serde_json::to_writer_pretty(&mut lock, value)?;
    writeln!(lock)?;
    Ok(())
}

impl TryFrom<PageArgs> for PageQuery {
    type Error = ValidationError;

    fn try_from(args: PageArgs) -> Result<Self, Self::Error> {
        Self::new(args.page_index, args.page_size)
    }
}

impl TryFrom<SecuritiesPageArgs> for PageQuery {
    type Error = ValidationError;

    fn try_from(args: SecuritiesPageArgs) -> Result<Self, Self::Error> {
        Self::new(args.page_index, args.page_size)
    }
}

impl TryFrom<SecuritiesArgs> for SecuritiesQuery {
    type Error = ValidationError;

    fn try_from(args: SecuritiesArgs) -> Result<Self, Self::Error> {
        Self::new(
            args.market.map(|market| market.as_str().to_owned()),
            args.page.try_into()?,
        )
    }
}

impl TryFrom<SecuritiesDetailsArgs> for SecuritiesDetailsQuery {
    type Error = ValidationError;

    fn try_from(args: SecuritiesDetailsArgs) -> Result<Self, Self::Error> {
        Self::new(
            args.market.map(|market| market.as_str().to_owned()),
            args.symbol,
            args.page.try_into()?,
        )
    }
}

impl TryFrom<IndexComponentsArgs> for IndexComponentsQuery {
    type Error = ValidationError;

    fn try_from(args: IndexComponentsArgs) -> Result<Self, Self::Error> {
        Self::new(args.index_code, args.page.try_into()?)
    }
}

impl TryFrom<IndexListArgs> for IndexListQuery {
    type Error = ValidationError;

    fn try_from(args: IndexListArgs) -> Result<Self, Self::Error> {
        Self::new(
            args.exchange.map(|market| market.as_str().to_owned()),
            args.page.try_into()?,
        )
    }
}

impl TryFrom<DailyOhlcArgs> for DailyOhlcQuery {
    type Error = ValidationError;

    fn try_from(args: DailyOhlcArgs) -> Result<Self, Self::Error> {
        Self::parse(DailyOhlcInput {
            symbol: args.symbol,
            from_date: args.from_date,
            to_date: args.to_date,
            page: args.page.try_into()?,
            ascending: args.ascending,
        })
    }
}

impl TryFrom<IntradayOhlcArgs> for IntradayOhlcQuery {
    type Error = ValidationError;

    fn try_from(args: IntradayOhlcArgs) -> Result<Self, Self::Error> {
        Self::parse(IntradayOhlcInput {
            symbol: args.symbol,
            from_date: args.from_date.unwrap_or_default(),
            to_date: args.to_date.unwrap_or_default(),
            page: args.page.try_into()?,
            ascending: args.ascending,
            resolution: args.resolution,
        })
    }
}

impl TryFrom<DailyIndexArgs> for DailyIndexQuery {
    type Error = ValidationError;

    fn try_from(args: DailyIndexArgs) -> Result<Self, Self::Error> {
        Self::parse(DailyIndexInput {
            request_id: args.request_id,
            index_id: args.index_id,
            from_date: args.from_date,
            to_date: args.to_date,
            page: args.page.try_into()?,
            order_by: args.order_by,
            order: args.order.as_str().to_owned(),
        })
    }
}

impl TryFrom<DailyStockPriceArgs> for DailyStockPriceQuery {
    type Error = ValidationError;

    fn try_from(args: DailyStockPriceArgs) -> Result<Self, Self::Error> {
        let page = PageQuery::new(args.page_index, args.page_size)?;
        Self::parse(DailyStockPriceInput {
            symbol: args.symbol,
            from_date: args.from_date,
            to_date: args.to_date,
            page,
            market: args.market.map(|market| market.as_str().to_owned()),
        })
    }
}

impl TryFrom<BacktestArgs> for ApiRequest {
    type Error = ValidationError;

    fn try_from(args: BacktestArgs) -> Result<Self, Self::Error> {
        Ok(Self::Backtest(BacktestQuery::new(
            args.selected_date,
            args.symbol,
        )?))
    }
}
