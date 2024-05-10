/*
 * Copyright (c) Johannes Grimm 2024.
 */

use crate::prisma::PrismaClient;
use crate::route::health_check::health_check;
use crate::route::users::user_controller_init;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::config::PersistentSession;
use actix_session::storage::RedisActorSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::cookie::time::Duration;
use actix_web::dev::Server;
use actix_web::web::{scope, ServiceConfig};
use actix_web::{error, get, middleware, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;
use crate::route::auth::auth_controller_init;

#[allow(dead_code)]
async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("Not Found")
}

#[get("/")]
async fn index(identity: Option<Identity>, session: Session) -> actix_web::Result<impl Responder> {
    // let user_id: Option<String> = session.get::<String>("user_id").unwrap();
    let counter: i32 = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .unwrap_or(0);

    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    Ok(HttpResponse::Ok().body(format!("Hello {id}, session: {counter}")))
}

#[doc = "Setup the service served by the application."]
fn get_config(conf: &mut ServiceConfig) {
    conf.service(
        scope("/api")
            .service(health_check)
            .configure(user_controller_init)
            .configure(auth_controller_init),
    );
}

#[doc = "Create the server instance."]
pub async fn run(tcp_listener: TcpListener, data: PrismaClient) -> Result<Server, std::io::Error> {
    let data = web::Data::new(data);
    let private_key = actix_web::cookie::Key::generate();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("127.0.0.1:6379"),
                    private_key.clone(),
                )
                .cookie_name("plnt_auth".to_owned())
                // .cookie_secure(secure) // Requires HTTPS
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(24)))
                .build(),
            )
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .default_service(web::route().to(not_found))
            .service(index)
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
