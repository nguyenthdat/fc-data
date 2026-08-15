use serde::{Deserialize, Serialize};
use serde_json::Number;

/// One intraday market tick with ten levels of order-book depth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IntradayByTick {
    /// Security symbol.
    pub symbol: String,
    /// Opening price.
    pub open: Option<Number>,
    /// Highest price.
    pub high: Option<Number>,
    /// Lowest price.
    pub low: Option<Number>,
    /// Closing price.
    pub close: Option<Number>,
    /// Trading date.
    pub trading_date: String,
    /// Tick time.
    pub time: String,
    /// Traded volume.
    pub volume: Option<Number>,
    /// Best ask price.
    pub ask_price_1: Option<Number>,
    /// Second ask price.
    pub ask_price_2: Option<Number>,
    /// Third ask price.
    pub ask_price_3: Option<Number>,
    /// Fourth ask price.
    pub ask_price_4: Option<Number>,
    /// Fifth ask price.
    pub ask_price_5: Option<Number>,
    /// Sixth ask price.
    pub ask_price_6: Option<Number>,
    /// Seventh ask price.
    pub ask_price_7: Option<Number>,
    /// Eighth ask price.
    pub ask_price_8: Option<Number>,
    /// Ninth ask price.
    pub ask_price_9: Option<Number>,
    /// Tenth ask price.
    pub ask_price_10: Option<Number>,
    /// Best ask volume.
    pub ask_vol_1: Option<Number>,
    /// Second ask volume.
    pub ask_vol_2: Option<Number>,
    /// Third ask volume.
    pub ask_vol_3: Option<Number>,
    /// Fourth ask volume.
    pub ask_vol_4: Option<Number>,
    /// Fifth ask volume.
    pub ask_vol_5: Option<Number>,
    /// Sixth ask volume.
    pub ask_vol_6: Option<Number>,
    /// Seventh ask volume.
    pub ask_vol_7: Option<Number>,
    /// Eighth ask volume.
    pub ask_vol_8: Option<Number>,
    /// Ninth ask volume.
    pub ask_vol_9: Option<Number>,
    /// Tenth ask volume.
    pub ask_vol_10: Option<Number>,
    /// Best bid price.
    pub bid_price_1: Option<Number>,
    /// Second bid price.
    pub bid_price_2: Option<Number>,
    /// Third bid price.
    pub bid_price_3: Option<Number>,
    /// Fourth bid price.
    pub bid_price_4: Option<Number>,
    /// Fifth bid price.
    pub bid_price_5: Option<Number>,
    /// Sixth bid price.
    pub bid_price_6: Option<Number>,
    /// Seventh bid price.
    pub bid_price_7: Option<Number>,
    /// Eighth bid price.
    pub bid_price_8: Option<Number>,
    /// Ninth bid price.
    pub bid_price_9: Option<Number>,
    /// Tenth bid price.
    pub bid_price_10: Option<Number>,
    /// Best bid volume.
    pub bid_vol_1: Option<Number>,
    /// Second bid volume.
    pub bid_vol_2: Option<Number>,
    /// Third bid volume.
    pub bid_vol_3: Option<Number>,
    /// Fourth bid volume.
    pub bid_vol_4: Option<Number>,
    /// Fifth bid volume.
    pub bid_vol_5: Option<Number>,
    /// Sixth bid volume.
    pub bid_vol_6: Option<Number>,
    /// Seventh bid volume.
    pub bid_vol_7: Option<Number>,
    /// Eighth bid volume.
    pub bid_vol_8: Option<Number>,
    /// Ninth bid volume.
    pub bid_vol_9: Option<Number>,
    /// Tenth bid volume.
    pub bid_vol_10: Option<Number>,
    /// Trade side.
    #[serde(rename = "side")]
    pub side: Option<String>,
    /// Absolute price change.
    #[serde(rename = "priceChange")]
    pub price_change: Option<Number>,
    /// Percentage price change.
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Option<Number>,
    /// Direction of the price change.
    #[serde(rename = "changeType")]
    pub change_type: Option<String>,
}
