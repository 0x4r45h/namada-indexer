use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum IbcAckStatus {
    Success,
    Fail,
    Timeout,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbcAck {
    pub status: IbcAckStatus,
    pub timeout: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbcRateLimit {
    pub token_address: String,
    pub throughput_limit: String,
}
