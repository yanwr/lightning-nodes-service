use chrono::{DateTime, Utc};
use sqlx::{PgPool, QueryBuilder, prelude::FromRow};

use crate::{errors::AppError, gateways::mempool::fetch_rankings_connectivity::MempoolNodeResponse};

#[derive(Debug, Clone, FromRow)]
pub struct Node {
    pub public_key: String,
    pub alias: String,
    pub capacity_sats: i64,
    pub first_seen: DateTime<Utc>,
}

impl Node {
    pub async fn replace(pool: &PgPool, nodes: &[Node]) -> Result<(), AppError> {
        let mut tx = pool.begin().await?;
        sqlx::query("TRUNCATE TABLE nodes")
            .execute(&mut *tx)
            .await?;
        let mut query_builder = QueryBuilder::new(
            "INSERT INTO nodes \
            (public_key, alias, capacity_sats, first_seen) "
        );
        query_builder.push_values(nodes, |mut row, node| {
            row.push_bind(&node.public_key)
                .push_bind(&node.alias)
                .push_bind(node.capacity_sats)
                .push_bind(node.first_seen);
        });
        query_builder
            .build()
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn list(pool: &PgPool) -> Result<Vec<Self>, AppError> {
        let nodes = sqlx::query_as::<_, Self>(
            r#"
            SELECT
                public_key,
                alias,
                capacity_sats,
                first_seen
            FROM nodes
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(nodes)
    }
}

impl TryFrom<MempoolNodeResponse> for Node {
    type Error = AppError;
    fn try_from(node: MempoolNodeResponse) -> Result<Self, Self::Error> {
        let first_seen = DateTime::from_timestamp(node.first_seen, 0)
            .ok_or_else(|| AppError::MempoolGatewayInvalidData(String::from("invalid firstSeen unix timestamp")))?;
        if node.capacity < 0 {
            return Err(AppError::MempoolGatewayInvalidData(String::from("capacity cannot be negative")));
        }
        Ok(Self {
            public_key: node.public_key,
            alias: node.alias,
            capacity_sats: node.capacity,
            first_seen,
        })
    }
}