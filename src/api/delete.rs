use actix_web::{Responder, web, delete};
use sea_orm::EntityTrait;
use serde::Deserialize;

use crate::{api::BaseResponse, AppState, database::{sentences, wallpaper, lyrics, images, auth}};

#[derive(Deserialize)]
pub struct DeleteBody {
    id: i32,
    auth_key: String,
}

#[delete("/{scope}/delete")]
pub async fn delete(scope: web::Path<String>, data: web::Data<AppState>, body: web::Form<DeleteBody>) -> impl Responder {
    let conn = &data.conn;
    let admins = auth::Entity::find()
        .all(conn)
        .await
        .unwrap();

    let mut admin: Option<auth::Model> = Option::from(None);
    for a in admins {
        if a.auth_key == body.auth_key {
            admin = Option::from(a);
            break;
        }
    };
    match admin {
        Some(data) => {
            if data.perm_level < 2 {
                return web::Json(
                    BaseResponse {
                        status: 400,
                        message: "权限不足".to_string(),
                        data: Option::from(None),
                    }
                );
            }
        },
        _ => {
            return web::Json(
                BaseResponse {
                    status: 400,
                    message: "无效的auth_key".to_string(),
                    data: None,
                }
            );
        }
    };
    match scope.as_str() {
        "images" => {
            images::Entity::delete_by_id(body.id)
                .exec(conn)
                .await
                .unwrap();
        },
        "lyrics" => {
            lyrics::Entity::delete_by_id(body.id)
                .exec(conn)
                .await
                .unwrap();
        },
        "wallpaper" => {
            wallpaper::Entity::delete_by_id(body.id)
                .exec(conn)
                .await
                .unwrap();
        },
        "sentences" => {
            sentences::Entity::delete_by_id(body.id)
                .exec(conn)
                .await
                .unwrap();
        },
        _ => {
            return web::Json(
                BaseResponse {
                    status: 400,
                    message: "错误的scope".to_string(),
                    data: None,
                }
            );
        } 
    }
    web::Json(
        BaseResponse {
            status: 0,
            message: "ok".to_string(),
            data: None
        }
    )
}