/*
 * Copyright (c) Johannes Grimm 2024.
 */

use std::env;

use ::sea_orm::*;
use sea_orm_migration::prelude::*;

use migration::sea_orm::{Database, DbBackend};
use migration::{ConnectionTrait, DbErr, MigratorTrait, SchemaManager};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[tokio::main]
async fn start() -> Result<(), DbErr> {
    println!("Hello World from the api crate!");
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");

    let db = Database::connect(&db_url)
        .await
        .expect("Database connection failed");

    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`", db_name),
            ))
            .await?;
            let db_url = format!("{}/{}", db_url, db_name);
            Database::connect(&db_url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            ))
            .await?;
            let db_url = format!("{}/{}", db_url, db_name);
            Database::connect(&db_url).await?
        }
        DbBackend::Sqlite => {
            let db_url = format!("{}/{}", db_url, db_name);
            Database::connect(&db_url).await?
        }
    };

    let schema_manager = SchemaManager::new(db);
    migration::Migrator::up(db, None).await?;

    assert!(schema_manager.has_table("post").await?);

    println!("Hello World from the api crate!");

    Ok(())
}

pub fn main() {
    let result = start();
    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/*use actix_web::{
    error, get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};

use dotenvy::dotenv;
use service::{
    sea_orm::{Database, DatabaseConnection},
    Mutation, Query,
};
use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use std::env;
use sea_orm::{ConnectionTrait, Statement};

#[get("/")]
async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!\r\n"
}

#[get("/show/{id}")]
async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}

#[tokio::main]
async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // establish connection to database
    let connection = Database::connect(&db_url)
        .await
        .expect("Could not connect to database server");
    //const res = await client.query(`SELECT datname FROM pg_catalog.pg_database WHERE datname = '${DB_NAME}'`);
    let res = connection.execute(Statement::from_string(
        connection.get_database_backend(),
        format!("SELECT datname FROM pg_catalog.pg_database WHERE datname = {db_name}"),
    ))
        .await;
    match res.err() {
        None => {}
        Some(_) => {
            // create database
            connection.execute(Statement::from_string(
                connection.get_database_backend(),
                format!("CREATE DATABASE {db_name}"),
            ))
                .await
                .expect("Could not create database");
        }
    }

    /*db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
    ))
        .await?;
    */
    connection.execute(Statement::from_string(
        connection.get_database_backend(),
        format!("CREATE DATABASE \"{}\";", db_name),
    ))
        .await?;
    let db_url = format!("{}/{}", db_url, db_name);
    let connection = &Database::connect(&db_url).await?;


    /*let db_url = format!("{}/{}", db_url, db_name);
    let connection = &Database::connect(&db_url)
        .await
        .expect("Could not connect to database");

    let conn = Database::connect(&db_url).await.unwrap();*/

    Migrator::up(&connection, None).await.unwrap();

    println!("Hello, world!");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(web::scope("/api").service(user_detail))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
*/
