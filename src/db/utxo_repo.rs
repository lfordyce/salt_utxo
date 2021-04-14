use crate::entity::repository::Repository;
use crate::entity::utxo::{Utxo, UtxoBalance};

use async_trait::async_trait;
use sqlx::PgPool;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct BtcRepo {
    pool: sqlx::PgPool,
}

impl BtcRepo {
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("could not create postgres connection pool");
        Self { pool }
    }
}

#[async_trait]
impl Repository for BtcRepo {
    async fn find_all(&self, limit: i32, offset: i32) -> Result<Vec<Utxo>, Box<dyn Error>> {
        let rows = sqlx::query_as::<_, Utxo>(
            "SELECT id, txid, address, amount, spent FROM btc_utxo LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_balance_by_address(
        &self,
        address: &str,
        spent: bool,
    ) -> Result<Option<UtxoBalance>, Box<dyn Error>> {
        let result =
            sqlx::query_as::<_, UtxoBalance>("SELECT sum(amount) AS balance, count(txid) AS tx_count FROM public.btc_utxo WHERE address = $1 AND spent = $2")
            .bind(address)
            .bind(spent)
            .fetch_optional(&self.pool).await?;
        Ok(result)
    }

    async fn create_utxo(&self, utxo: &Utxo) -> Result<(), Box<dyn Error>> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("INSERT INTO public.btc_utxo (id, txid, address, amount, spent) VALUES ($1, $2, $3, $4, $5)")
            .bind(&utxo.id)
            .bind(&utxo.txid)
            .bind(&utxo.address)
            .bind(&utxo.amount)
            .bind(&utxo.spent)
            .execute(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
}
