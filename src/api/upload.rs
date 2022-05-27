use actix_web::{Responder, web, post};
use sea_orm::{EntityTrait, ActiveValue};
use serde::Deserialize;

use chrono::prelude::*;

use crate::{api::BaseResponse, AppState, database::{sentences, wallpaper, lyrics, images, auth}};

#[derive(Deserialize)]
pub struct PostBody {
    content: String,
    author: Option<String>,
    work: Option<String>,
    auth_key: String
}

#[post("/{scope}/upload")]
pub async fn upload(scope: web::Path<String>, data: web::Data<AppState>, body: web::Form<PostBody>) -> impl Responder {
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
            if data.perm_level < 1 {
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
            images::Entity::insert(images::ActiveModel {
                id: ActiveValue::NotSet,
                author: ActiveValue::Set(body.author.clone()),
                content: ActiveValue::Set(body.content.clone()),
                created_at: ActiveValue::Set(Local::now().naive_local()),
            })
            .exec(conn)
            .await
            .unwrap();
        },
        "lyrics" => {
            lyrics::Entity::insert(lyrics::ActiveModel {
                id: ActiveValue::NotSet,
                work: ActiveValue::Set(body.work.clone()),
                author: ActiveValue::Set(body.author.clone()),
                content: ActiveValue::Set(body.content.clone()),
                created_at: ActiveValue::Set(Local::now().naive_local()),
            })
            .exec(conn)
            .await
            .unwrap();
        },
        "wallpaper" => {
            wallpaper::Entity::insert(wallpaper::ActiveModel {
                id: ActiveValue::NotSet,
                author: ActiveValue::Set(body.author.clone()),
                content: ActiveValue::Set(body.content.clone()),
                created_at: ActiveValue::Set(Local::now().naive_local()),
            })
            .exec(conn)
            .await
            .unwrap();
        },
        "sentences" => {
            sentences::Entity::insert(sentences::ActiveModel {
                id: ActiveValue::NotSet,
                work: ActiveValue::Set(body.work.clone()),
                author: ActiveValue::Set(body.author.clone()),
                content: ActiveValue::Set(body.content.clone()),
                created_at: ActiveValue::Set(Local::now().naive_local()),
            })
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