use log::error;
use rocket::{get, post};
use rocket::http::Status;
use rocket_okapi::openapi;
use rocket::serde::json::Json;

use crate::domain::greetings::{Greeting, NewGreeting};
use crate::database::DbConn;

#[openapi]
#[get("/greetings")]
pub async fn get_greetings(conn: DbConn) -> Result<Json<Vec<Greeting>>, Status> {
  conn
    .run(|c| {
      Greeting::load(c)
        .and_then(|g| Ok(Json(g)))
        .or_else(|e| {
          error!("{}", e);
          Err(Status::InternalServerError)
        })
    })
    .await
}

#[openapi]
#[post("/greetings", data = "<value>")]
pub async fn add_greeting(
  conn: DbConn,
  value: Json<NewGreeting>,
) -> Result<Json<Greeting>, Status> {
  conn
    .run(|c| {
      Greeting::create(value.into_inner(), c)
        .and_then(|g| Ok(Json(g)))
        .or_else(|e| {
          error!("{}", e);
          Err(Status::InternalServerError)
        })
    })
    .await
}

