use serde::Deserialize;

/// Securities trading status broadcast (`F`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecuritiesStatus {
    /// Wire record type.
    #[serde(alias = "Rtype")]
    pub r_type: String,
    /// SSI market identifier.
    pub market_id: String,
    /// Trading date.
    pub trading_date: String,
    /// Broadcast time.
    pub time: String,
    /// Security symbol.
    pub symbol: String,
    /// Trading session code.
    pub trading_session: String,
    /// Trading status code.
    pub trading_status: String,
    /// Exchange code.
    pub exchange: String,
    /// Odd-lot trading session code.
    #[serde(default)]
    pub trading_ol_session: Option<String>,
}

/// Matched-trade broadcast (`X-TRADE`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Trade {
    /// Wire record type.
    #[serde(alias = "Rtype")]
    pub r_type: String,
    /// Trading date.
    pub trading_date: String,
    /// Broadcast time.
    pub time: String,
    /// ISIN or SSI instrument identifier.
    #[serde(default, alias = "ISIN", alias = "ISin")]
    pub isin: Option<String>,
    /// Security symbol.
    pub symbol: String,
    /// Ceiling price.
    pub ceiling: f64,
    /// Floor price.
    pub floor: f64,
    /// Reference price.
    pub ref_price: f64,
    /// Average matched price.
    pub avg_price: f64,
    /// Previous closing value.
    pub prior_val: f64,
    /// Latest matched price.
    pub last_price: f64,
    /// Latest matched volume.
    pub last_vol: f64,
    /// Cumulative matched value.
    pub total_val: f64,
    /// Cumulative matched volume.
    pub total_vol: f64,
    /// SSI market identifier.
    pub market_id: String,
    /// Exchange code.
    pub exchange: String,
    /// Trading session code.
    pub trading_session: String,
    /// Trading status code.
    pub trading_status: String,
    /// Price change.
    pub change: f64,
    /// Percentage price change.
    pub ratio_change: f64,
    /// Estimated matched price.
    pub est_matched_price: f64,
    /// Session high.
    pub highest: f64,
    /// Session low.
    pub lowest: f64,
    /// Matched side code.
    pub side: String,
}

/// Foreign ownership room broadcast (`R`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ForeignRoom {
    /// Wire record type.
    #[serde(alias = "Rtype")]
    pub r_type: String,
    /// Trading date.
    pub trading_date: String,
    /// Broadcast time.
    pub time: String,
    /// ISIN or SSI instrument identifier.
    #[serde(alias = "ISIN", alias = "ISin")]
    pub isin: String,
    /// Security symbol.
    pub symbol: String,
    /// Total foreign room.
    pub total_room: f64,
    /// Remaining foreign room.
    pub current_room: f64,
    /// Foreign buy volume.
    pub buy_vol: f64,
    /// Foreign sell volume.
    pub sell_vol: f64,
    /// Foreign buy value.
    pub buy_val: f64,
    /// Foreign sell value.
    pub sell_val: f64,
    /// SSI market identifier.
    pub market_id: String,
    /// Exchange code.
    pub exchange: String,
}

/// Market-index broadcast (`MI`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketIndex {
    /// Index identifier.
    pub index_id: String,
    /// Estimated index value when supplied by SSI.
    #[serde(default)]
    pub index_val_est: Option<f64>,
    /// Current index value.
    pub index_value: f64,
    /// Previous index value.
    pub prior_index_value: f64,
    /// Trading date.
    pub trading_date: String,
    /// Broadcast time.
    pub time: String,
    /// Total matched trades.
    pub total_trade: f64,
    /// Normal-order matched quantity.
    pub total_qtty: f64,
    /// Normal-order matched value.
    pub total_value: f64,
    /// Index name.
    pub index_name: String,
    /// Advancing constituent count.
    pub advances: f64,
    /// Unchanged constituent count.
    pub no_changes: f64,
    /// Declining constituent count.
    pub declines: f64,
    /// Ceiling constituent count.
    pub ceilings: f64,
    /// Floor constituent count.
    pub floors: f64,
    /// Index change.
    pub change: f64,
    /// Percentage index change.
    pub ratio_change: f64,
    /// Put-through matched quantity.
    pub total_qtty_pt: f64,
    /// Put-through matched value.
    pub total_value_pt: f64,
    /// Exchange code.
    pub exchange: String,
    /// Quantity across normal and put-through orders.
    pub all_qty: f64,
    /// Value across normal and put-through orders.
    pub all_value: f64,
    /// Index classification.
    #[serde(default)]
    pub index_type: Option<String>,
    /// Trading session code.
    #[serde(default)]
    pub trading_session: Option<String>,
    /// SSI market identifier.
    #[serde(default)]
    pub market_id: Option<String>,
    /// Wire record type.
    #[serde(alias = "Rtype")]
    pub r_type: String,
    /// Odd-lot matched quantity.
    pub total_qtty_od: f64,
    /// Odd-lot matched value.
    pub total_value_od: f64,
}

/// Realtime OHLCV broadcast (`B`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RealtimeBar {
    /// Wire record type.
    #[serde(alias = "Rtype")]
    pub r_type: String,
    /// Trading date.
    pub trading_date: String,
    /// Broadcast time.
    #[serde(alias = "TradingTime")]
    pub time: String,
    /// Security symbol.
    pub symbol: String,
    /// Opening price.
    pub open: f64,
    /// Session high.
    pub high: f64,
    /// Session low.
    pub low: f64,
    /// Latest closing price.
    pub close: f64,
    /// Latest matched volume.
    pub volume: f64,
    /// Latest matched value.
    pub value: f64,
}
