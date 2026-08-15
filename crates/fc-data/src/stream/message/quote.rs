use serde::Deserialize;

/// Ten-level order-book broadcast (`X-QUOTE`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(from = "QuoteWire")]
pub struct Quote {
    /// Trading date.
    pub trading_date: String,
    /// Broadcast time.
    pub time: String,
    /// Exchange code.
    pub exchange: String,
    /// Security symbol.
    pub symbol: String,
    /// Wire record type.
    pub r_type: String,
    /// SSI stock number when supplied by the quote feed.
    pub stock_no: Option<String>,
    /// SSI stock classification when supplied by the quote feed.
    pub stock_type: Option<String>,
    /// Ask prices from level 1 through 10.
    pub ask_prices: [f64; 10],
    /// Ask volumes from level 1 through 10.
    pub ask_volumes: [f64; 10],
    /// Bid prices from level 1 through 10.
    pub bid_prices: [f64; 10],
    /// Bid volumes from level 1 through 10.
    pub bid_volumes: [f64; 10],
    /// Trading session code.
    pub trading_session: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct QuoteWire {
    trading_date: String,
    time: String,
    exchange: String,
    symbol: String,
    #[serde(alias = "Rtype")]
    r_type: String,
    #[serde(default)]
    stock_no: Option<String>,
    #[serde(default)]
    stock_type: Option<String>,
    ask_price1: f64,
    ask_price2: f64,
    ask_price3: f64,
    ask_price4: f64,
    ask_price5: f64,
    ask_price6: f64,
    ask_price7: f64,
    ask_price8: f64,
    ask_price9: f64,
    ask_price10: f64,
    ask_vol1: f64,
    ask_vol2: f64,
    ask_vol3: f64,
    ask_vol4: f64,
    ask_vol5: f64,
    ask_vol6: f64,
    ask_vol7: f64,
    ask_vol8: f64,
    ask_vol9: f64,
    ask_vol10: f64,
    bid_price1: f64,
    bid_price2: f64,
    bid_price3: f64,
    bid_price4: f64,
    bid_price5: f64,
    bid_price6: f64,
    bid_price7: f64,
    bid_price8: f64,
    bid_price9: f64,
    bid_price10: f64,
    bid_vol1: f64,
    bid_vol2: f64,
    bid_vol3: f64,
    bid_vol4: f64,
    bid_vol5: f64,
    bid_vol6: f64,
    bid_vol7: f64,
    bid_vol8: f64,
    bid_vol9: f64,
    bid_vol10: f64,
    trading_session: String,
}

impl From<QuoteWire> for Quote {
    fn from(wire: QuoteWire) -> Self {
        Self {
            trading_date: wire.trading_date,
            time: wire.time,
            exchange: wire.exchange,
            symbol: wire.symbol,
            r_type: wire.r_type,
            stock_no: wire.stock_no,
            stock_type: wire.stock_type,
            ask_prices: [
                wire.ask_price1,
                wire.ask_price2,
                wire.ask_price3,
                wire.ask_price4,
                wire.ask_price5,
                wire.ask_price6,
                wire.ask_price7,
                wire.ask_price8,
                wire.ask_price9,
                wire.ask_price10,
            ],
            ask_volumes: [
                wire.ask_vol1,
                wire.ask_vol2,
                wire.ask_vol3,
                wire.ask_vol4,
                wire.ask_vol5,
                wire.ask_vol6,
                wire.ask_vol7,
                wire.ask_vol8,
                wire.ask_vol9,
                wire.ask_vol10,
            ],
            bid_prices: [
                wire.bid_price1,
                wire.bid_price2,
                wire.bid_price3,
                wire.bid_price4,
                wire.bid_price5,
                wire.bid_price6,
                wire.bid_price7,
                wire.bid_price8,
                wire.bid_price9,
                wire.bid_price10,
            ],
            bid_volumes: [
                wire.bid_vol1,
                wire.bid_vol2,
                wire.bid_vol3,
                wire.bid_vol4,
                wire.bid_vol5,
                wire.bid_vol6,
                wire.bid_vol7,
                wire.bid_vol8,
                wire.bid_vol9,
                wire.bid_vol10,
            ],
            trading_session: wire.trading_session,
        }
    }
}
