use crate::config::Config;
use crate::handlers::ws::ws_broadcast;
use crate::state::AppState;
use axum::extract::{Json, State};
use axum_typed_multipart::BaseMultipart;
use model::cargo::*;
use model::util::{ApiError, ApiResponse};
use model::ws_msg::WSMsg;
use utils::texture::generate_texture;

pub async fn get_cargoes(State(app_state): State<AppState>) -> Json<ApiResponse<Vec<Cargo>>> {
    ApiResponse::new_success(Cargo::get_20(&app_state.pool).await).into()
}

pub async fn get_today_cargoes(State(app_state): State<AppState>) -> Json<ApiResponse<Vec<Cargo>>> {
    ApiResponse::new_success(Cargo::get_today(&app_state.pool).await).into()
}

pub async fn send_cargo(
    State(app_state): State<AppState>,
    data: BaseMultipart<CargoRequest, ApiError>,
) -> Json<ApiResponse<Cargo>> {
    let CargoRequest {
        paint_time,
        cargo_kind,
        file,
    } = data.data;

    let cargo = Cargo::create(
        &app_state.pool,
        CargoInput {
            paint_time,
            kind: cargo_kind.clone(),
        },
    )
    .await;

    let Cargo { id, .. } = cargo;

    let path = format!("{}/backend/db/storage", app_state.config.root_dir);
    generate_texture(id, &file, &path);

    let Config { host, port, .. } = &app_state.config;

    ws_broadcast(
        WSMsg::cargo(
            cargo_kind,
            id,
            &format!("https://tsps-ntpc.2enter.art/api/storage/texture/{id}.jpg"),
        ),
        &app_state.ws_sender,
    );

    ApiResponse::new_success(cargo).into()
}
