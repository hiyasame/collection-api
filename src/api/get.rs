use actix_web::{get, Responder, web};
use sea_orm::EntityTrait;
use serde::Deserialize;
use serde_json::json;

use crate::{api::BaseResponse, AppState, database::{sentences, wallpaper, lyrics, images}};

#[derive(Deserialize)]
pub struct GetQuery {
    id: i32,
}

#[get("/{scope}/get")]
pub async fn get(scope: web::Path<String>, data: web::Data<AppState>, query: web::Query<GetQuery>) -> impl Responder {
    let conn = &data.conn;
    match scope.as_str() {
        "images" => {
            let data = images::Entity::find_by_id(query.id)
                .one(conn)
                .await
                .unwrap();
            let data = match data {
                Some(data) => Option::from(json!(data)),
                None => return web::Json(
                    BaseResponse {
                        status: 1,
                        message: "not found".to_string(),
                        data: None,
                    }
                ),
            };
            web::Json(
                BaseResponse {
                    status: 0,
                    message: "ok".to_string(),
                    data,
                }
            )
        },
        "lyrics" => {
            let data = lyrics::Entity::find_by_id(query.id)
                .one(conn)
                .await
                .unwrap();
            let data = match data {
                Some(data) => Option::from(json!(data)),
                None => return web::Json(
                    BaseResponse {
                        status: 1,
                        message: "not found".to_string(),
                        data: None,
                    }
                ),
            };
            web::Json(
                BaseResponse {
                    status: 0,
                    message: "ok".to_string(),
                    data,
                }
            )
        },
        "wallpaper" => {
            let data = wallpaper::Entity::find_by_id(query.id)
                .one(conn)
                .await
                .unwrap();
            let data = match data {
                Some(data) => Option::from(json!(data)),
                None => return web::Json(
                    BaseResponse {
                        status: 1,
                        message: "not found".to_string(),
                        data: None,
                    }
                ),
            };
            web::Json(
                BaseResponse {
                    status: 0,
                    message: "ok".to_string(),
                    data,
                }
            )
        },
        "sentences" => {
            let data = sentences::Entity::find_by_id(query.id)
                .one(conn)
                .await
                .unwrap();
            let data = match data {
                Some(data) => Option::from(json!(data)),
                None => return web::Json(
                    BaseResponse {
                        status: 1,
                        message: "not found".to_string(),
                        data: None,
                    }
                ),
            };
            web::Json(
                BaseResponse {
                    status: 0,
                    message: "ok".to_string(),
                    data,
                }
            )
        },
        _ => web::Json(
            BaseResponse {
                status: 400,
                message: "错误的scope".to_string(),
                data: None,
            }
        )
    }
}