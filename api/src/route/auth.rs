/*
 * Copyright (c) Johannes Grimm 2024.
 */

use crate::model::dto::auth::LoginRequest;
use crate::prisma::PrismaClient;
use crate::service::authentication::login_user;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub fn auth_controller_init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(web::scope("/auth"));
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct IndexResponse {
    user_id: Option<String>,
    counter: i32,
}

#[post("/login")]
async fn login(
    body: web::Json<LoginRequest>,
    req: HttpRequest,
    data: web::Data<PrismaClient>,
) -> actix_web::Result<HttpResponse> {
    let login_result = login_user(body.into_inner(), data);

    //    Identity::login(&req.extensions(), id.clone()).unwrap();

    //  Ok(HttpResponse::Ok().json(IndexResponse {
    //    user_id: Some(id),
    //    counter,
    //}))
}

#[post("/logout")]
async fn logout(ident: Identity) -> actix_web::Result<String> {
    ident.logout();
    Ok("logged out".to_owned())
}

#[get("/do_something")]
async fn do_something(session: Session) -> actix_web::Result<HttpResponse> {
    let user_id: Option<String> = session.get::<String>("user_id").unwrap();
    let counter: i32 = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .map_or(1, |inner| inner + 1);
    session.insert("counter", counter)?;

    Ok(HttpResponse::Ok().json(IndexResponse { user_id, counter }))
}
