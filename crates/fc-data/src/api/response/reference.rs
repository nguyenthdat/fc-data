use serde::{Deserialize, Serialize};

/// Security record returned by the securities endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Security {
    /// SSI market code.
    pub market: String,
    /// English security name when supplied.
    pub stock_en_name: Option<String>,
    /// Local security name when supplied.
    pub stock_name: Option<String>,
    /// Security symbol.
    pub symbol: String,
}

/// Securities details report returned by SSI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecuritiesDetails {
    /// SSI report type.
    pub r_type: String,
    /// Detailed security records.
    pub repeated_info: Vec<SecurityDetails>,
    /// SSI report date.
    pub report_date: String,
    /// Total symbols represented by the report.
    pub total_no_sym: String,
}

/// Detailed security metadata from a securities details report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecurityDetails {
    /// Contract multiplier.
    pub contract_multiplier: String,
    /// Exercise ratio using SSI's captured spelling.
    pub excercise_ratio: String,
    /// Exchange code.
    pub exchange: String,
    /// Exercise price.
    pub exercise_price: String,
    /// Exercise style.
    pub exercise_style: String,
    /// First trading date.
    pub first_trading_date: String,
    /// International securities identifier when supplied.
    pub isin: Option<String>,
    /// Issue date.
    pub issue_date: String,
    /// Issuer when supplied.
    pub issuer: Option<String>,
    /// Last trading date.
    pub last_trading_date: String,
    /// Listed share count.
    pub listed_share: String,
    /// Trading lot size.
    pub lot_size: String,
    /// SSI market identifier.
    pub market_id: String,
    /// Maturity date.
    pub maturity_date: String,
    /// Put-or-call classification when supplied.
    pub put_or_call: Option<String>,
    /// Security type.
    pub sec_type: String,
    /// Settlement method.
    pub settl_method: String,
    /// Security symbol.
    pub symbol: String,
    /// English security name.
    pub symbol_eng_name: String,
    /// Local security name.
    pub symbol_name: String,
    /// First tick increment.
    pub tick_increment1: String,
    /// Second tick increment.
    pub tick_increment2: String,
    /// Third tick increment.
    pub tick_increment3: String,
    /// Fourth tick increment when supplied.
    pub tick_increment4: Option<String>,
    /// First tick threshold price.
    pub tick_price1: String,
    /// Second tick threshold price.
    pub tick_price2: String,
    /// Third tick threshold price.
    pub tick_price3: String,
    /// Fourth tick threshold price when supplied.
    pub tick_price4: Option<String>,
    /// Underlying symbol when supplied.
    pub underlying: Option<String>,
}

/// Index and its component records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndexComponents {
    /// Exchange code.
    pub exchange: String,
    /// Index code.
    pub index_code: String,
    /// Constituent securities.
    pub index_component: Vec<IndexComponent>,
    /// Index name.
    pub index_name: String,
    /// Total constituent count.
    pub total_symbol_no: String,
}

/// One index constituent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndexComponent {
    /// International securities identifier.
    pub isin: String,
    /// Constituent stock symbol.
    pub stock_symbol: String,
}

/// Index-list record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Index {
    /// Exchange code.
    pub exchange: String,
    /// Index code.
    pub index_code: String,
    /// Index name when supplied.
    pub index_name: Option<String>,
}
