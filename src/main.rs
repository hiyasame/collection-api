use actix_web::{web, App, HttpServer};
use dotenv::{ dotenv, var };
use sea_orm::{DatabaseConnection, DbBackend, Schema, ConnectionTrait};

mod api;
pub mod database;

#[derive(Debug, Clone)]
pub struct AppState {
  conn: DatabaseConnection,
}

fn db_setup() -> String {
    dotenv().ok();
    var("DATABASE_URL").expect("DATABASE_URL is not set in .env file")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db_url = db_setup();
    let conn = sea_orm::Database::connect(db_url).await.unwrap();
    println!("Database connected!");

    match create_tables(&conn).await {
        Ok(_) => println!("Tables created!"),
        Err(_) => println!("Error creating tables")
    }

    let state = web::Data::new(AppState { conn });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(api::list::list)
            .service(api::get::get)
            .service(api::delete::delete)
            .service(api::upload::upload)
            .service(api::random::random)
    })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}

async fn create_tables(conn: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let db_mysql = DbBackend::MySql;
    let schema = Schema::new(db_mysql);
    // 全部语句先尝试执行一次，再集中处理异常
    let all = vec![
        conn.execute(
            db_mysql.build(&schema.create_table_from_entity(database::auth::Entity))
        ).await,
        conn.execute(
            db_mysql.build(&schema.create_table_from_entity(database::images::Entity))
        ).await,
        conn.execute(
            db_mysql.build(&schema.create_table_from_entity(database::lyrics::Entity))
        ).await,
        conn.execute(
            db_mysql.build(&schema.create_table_from_entity(database::sentences::Entity))
        ).await,
        conn.execute(
            db_mysql.build(&schema.create_table_from_entity(database::wallpaper::Entity))
        ).await,
    ];
    for ele in all {
        ele?;
    }
    Ok(())
}