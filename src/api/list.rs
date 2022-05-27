use actix_web::{get, Responder, web};
use sea_orm::{EntityTrait, QuerySelect, QueryTrait};
use serde::Deserialize;
use serde_json::json;

use crate::{api::BaseResponse, AppState, database::{sentences, wallpaper, lyrics, images}};

#[derive(Deserialize)]
pub struct ListQuery {
    limit: u64,
    offset: u64,
}

#[get("/{scope}/list")]
pub async fn list(scope: web::Path<String>, data: web::Data<AppState>, query: web::Query<ListQuery>) -> impl Responder {
    let conn = &data.conn;
    match scope.as_str() {
        "images" => {
            let data = images::Entity::find()
                .from_raw_sql(
                    images::Entity::find()
                            .limit(query.limit)
                            .offset(query.offset)
                            .build(sea_orm::DatabaseBackend::MySql)
                )
                .all(conn)
                .await
                .unwrap();
            web::Json(
                BaseResponse {
                    status: 0,
                    message: "ok".to_string(),
                    data: Option::from(json!(data)),
                }
            )
        },
        "lyrics" => {
            let data = lyrics::Entity::find()
                .from_raw_sql(
                    lyrics::Entity::find()
                            .limit(query.limit)
                            .offset(query.offset)
                            .build(sea_orm::DatabaseBackend::MySql)
                )
                .all(conn)
                .await
                .unwrap();
            web::Json(
                BaseResponse {
                    status: 0,
                    message: "ok".to_string(),
                    data: Option::from(json!(data)),
                }
            )
        },
        "wallpaper" => {
            let data = wallpaper::Entity::find()
                .from_raw_sql(
                    wallpaper::Entity::find()
                            .limit(query.limit)
                            .offset(query.offset)
                            .build(sea_orm::DatabaseBackend::MySql)
                )
                .all(conn)
                .await
                .unwrap();
            web::Json(
                BaseResponse {
                    status: 0,
                    message: "ok".to_string(),
                    data: Option::from(json!(data)),
                }
            )
        },
        "sentences" => {
            let data = sentences::Entity::find()
                .from_raw_sql(
                    sentences::Entity::find()
                            .limit(query.limit)
                            .offset(query.offset)
                            .build(sea_orm::DatabaseBackend::MySql)
                )
                .all(conn)
                .await
                .unwrap();
            web::Json(
                BaseResponse {
                    status: 0,
                    message: "ok".to_string(),
                    data: Option::from(json!(data)),
                }
            )
        },
        _ => web::Json(
            BaseResponse {
                status: 400,
                message: "错误的scope".to_string(),
                data: None,
            }
        ),
    }
}