use serde::{Deserialize, Deserializer, Serialize};

/// Uniform SSI REST response envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "T: Deserialize<'de>"))]
pub struct RestResponse<T> {
    /// Endpoint payload records.
    #[serde(deserialize_with = "null_as_empty")]
    pub data: Vec<T>,
    /// Human-readable SSI response message.
    pub message: String,
    /// SSI response status text.
    pub status: String,
    /// Total records reported by SSI.
    pub total_record: u64,
}

fn null_as_empty<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<Vec<T>>::deserialize(deserializer).map(Option::unwrap_or_default)
}
