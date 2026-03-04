use axum_typed_multipart::TryFromField;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, sqlx::Type, TryFromField, Clone)]
#[try_from_field(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase", type_name = "cargo_kind")]
pub enum CargoKind {
    Butterfly,
    Leaf,
    Cloud,
    Drip,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase", type_name = "cargo_status")]
pub enum CargoStatus {
    Shipping,
    Delivered,
    Launched,
}
