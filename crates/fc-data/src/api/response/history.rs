use serde::{Deserialize, Serialize};

/// Daily OHLC market record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DailyOhlc {
    /// Closing price.
    pub close: String,
    /// Highest price.
    pub high: String,
    /// Lowest price.
    pub low: String,
    /// SSI market code.
    pub market: String,
    /// Opening price.
    pub open: String,
    /// Security symbol.
    pub symbol: String,
    /// Time when supplied.
    pub time: Option<String>,
    /// Trading date.
    pub trading_date: String,
    /// Traded value.
    pub value: String,
    /// Traded volume.
    pub volume: String,
}

/// Intraday OHLC market record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IntradayOhlc {
    /// Closing price.
    pub close: String,
    /// Highest price.
    pub high: String,
    /// Lowest price.
    pub low: String,
    /// Opening price.
    pub open: String,
    /// Security symbol.
    pub symbol: String,
    /// Intraday time.
    pub time: String,
    /// Trading date.
    pub trading_date: String,
    /// Traded value.
    pub value: String,
    /// Traded volume.
    pub volume: String,
}

/// Daily index market record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DailyIndex {
    /// Advancing security count.
    pub advances: String,
    /// Ceiling security count.
    pub ceilings: String,
    /// Index value change.
    pub change: String,
    /// Declining security count.
    pub declines: String,
    /// Floor security count.
    pub floors: String,
    /// Index identifier.
    pub index_id: String,
    /// Index name.
    pub index_name: String,
    /// Index value.
    pub index_value: String,
    /// Unchanged security count.
    pub no_changes: String,
    /// Ratio change.
    pub ratio_change: String,
    /// Time when supplied.
    pub time: Option<String>,
    /// Total negotiated-deal value.
    pub total_deal_val: String,
    /// Total negotiated-deal volume.
    pub total_deal_vol: String,
    /// Total matched value.
    pub total_match_val: String,
    /// Total matched volume.
    pub total_match_vol: String,
    /// Total trade count.
    pub total_trade: String,
    /// Total traded value.
    pub total_val: String,
    /// Total traded volume.
    pub total_vol: String,
    /// Trading date.
    pub trading_date: String,
    /// Trading session.
    pub trading_session: String,
    /// Index type when supplied.
    pub type_index: Option<String>,
}

/// Daily stock price market record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DailyStockPrice {
    /// Average matched price.
    pub average_price: String,
    /// Ceiling price.
    pub ceiling_price: String,
    /// Closing price.
    pub close_price: String,
    /// Adjusted closing price.
    pub close_price_adjusted: String,
    /// Floor price.
    pub floor_price: String,
    /// Total foreign buy value.
    pub foreign_buy_val_total: String,
    /// Total foreign buy volume.
    pub foreign_buy_vol_total: String,
    /// Current foreign ownership room.
    pub foreign_current_room: String,
    /// Total foreign sell value.
    pub foreign_sell_val_total: String,
    /// Total foreign sell volume.
    pub foreign_sell_vol_total: String,
    /// Highest price.
    pub highest_price: String,
    /// Lowest price.
    pub lowest_price: String,
    /// Net foreign buy/sell value.
    pub net_buy_sell_val: String,
    /// Net foreign buy/sell volume.
    pub net_buy_sell_vol: String,
    /// Opening price.
    pub open_price: String,
    /// Percentage price change.
    pub per_price_change: String,
    /// Absolute price change.
    pub price_change: String,
    /// Reference price.
    pub ref_price: String,
    /// Security symbol.
    pub symbol: String,
    /// Time when supplied.
    pub time: Option<String>,
    /// Total buy trade count.
    pub total_buy_trade: String,
    /// Total buy trade volume.
    pub total_buy_trade_vol: String,
    /// Total negotiated-deal value.
    pub total_deal_val: String,
    /// Total negotiated-deal volume.
    pub total_deal_vol: String,
    /// Total matched value.
    pub total_match_val: String,
    /// Total matched volume.
    pub total_match_vol: String,
    /// Total sell trade count.
    pub total_sell_trade: String,
    /// Total sell trade volume.
    pub total_sell_trade_vol: String,
    /// Total traded value.
    pub total_traded_value: String,
    /// Total traded volume.
    pub total_traded_vol: String,
    /// Trading date.
    pub trading_date: String,
}
