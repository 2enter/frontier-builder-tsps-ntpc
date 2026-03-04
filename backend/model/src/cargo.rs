use crate::enums::{CargoKind, CargoStatus};
use axum::body::Bytes;
use axum_typed_multipart::TryFromMultipart;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, PgPool, Postgres};
use typeshare::typeshare;
use uuid::Uuid;

#[typeshare]
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Cargo {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub paint_time: i32,
    pub kind: CargoKind,
    pub status: CargoStatus,
}

#[typeshare]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargoInput {
    pub paint_time: f32,
    pub kind: CargoKind,
}

#[typeshare]
#[derive(TryFromMultipart, Serialize)]
#[serde(rename_all = "camelCase")]
#[try_from_multipart(rename_all = "camelCase")]
pub struct CargoRequest {
    pub cargo_kind: CargoKind,
    pub paint_time: f32,
    #[serde(skip)] // comment this line before generating typeshare types
    pub file: Bytes,
}

impl Cargo {
    pub async fn get_20(pool: &PgPool) -> Vec<Self> {
        sqlx::query_as("SELECT * FROM cargo ORDER BY created_at DESC LIMIT 20")
            .fetch_all(pool)
            .await
            .unwrap_or_default()
    }

    pub async fn get_today(pool: &PgPool) -> Vec<Self> {
        let today_start = Utc::now().date_naive();
        println!("{today_start}");
        sqlx::query_as("SELECT * FROM cargo WHERE created_at > $1")
            .bind(today_start)
            .fetch_all(pool)
            .await
            .unwrap_or_default()
    }

    pub async fn deliver(pool: &PgPool) -> Vec<Self> {
        let target_time = Utc::now() - chrono::Duration::seconds(60);
        sqlx::query_as(
            r#"UPDATE cargo SET status = 'delivered' WHERE status = 'shipping' and created_at < $1 RETURNING id;"#
        )
            .bind(target_time)
            .fetch_all(pool)
            .await
            .unwrap_or_default()
    }

    pub async fn launch(pool: &PgPool) -> usize {
        sqlx::query_as::<Postgres, Self>(
            "UPDATE cargo SET status = 'launched' WHERE status = 'delivered' RETURNING id;",
        )
        .fetch_all(pool)
        .await
        .unwrap_or_default()
        .len()
    }

    pub async fn create(pool: &PgPool, input: CargoInput) -> Self {
        query_as("INSERT INTO cargo (kind, paint_time) VALUES ($1, $2) RETURNING *;")
            .bind(input.kind)
            .bind(input.paint_time)
            .fetch_one(pool)
            .await
            .unwrap()
    }
}
