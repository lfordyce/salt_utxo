use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Utxo {
    pub id: Uuid,
    pub txid: String,
    pub address: String,
    pub amount: BigDecimal,
    pub spent: bool,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct UtxoBalance {
    pub balance: Option<BigDecimal>,
    pub tx_count: i64,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Address {
    pub address: String,
}
