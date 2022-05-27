use actix_web::{get, Responder, web};
use sea_orm::{EntityTrait, Statement};
use serde_json::json;

use crate::{api::BaseResponse, AppState, database::{sentences, wallpaper, lyrics, images}};

fn get_raw_sql(str: &str) -> Statement {
    Statement::from_string(
            sea_orm::DatabaseBackend::MySql, 
            format!("SELECT * FROM {str} as t1 WHERE t1.id>=(RAND()*(SELECT MAX(id) FROM {str}))LIMIT 1;")
    )
}

#[get("/{scope}/random")]
pub async fn random(scope: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;
    match scope.as_str() {
        "images" => {
            let data = images::Entity::find()
                .from_raw_sql(
                    get_raw_sql("images")
                )
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
            let data = lyrics::Entity::find()
                .from_raw_sql(
                    get_raw_sql("lyrics")
                )
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
            let data = wallpaper::Entity::find()
                .from_raw_sql(
                    get_raw_sql("wallpaper")
                )
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
            let data = sentences::Entity::find()
                .from_raw_sql(
                    get_raw_sql("sentences")
                )
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