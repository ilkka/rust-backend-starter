#[macro_use]
extern crate diesel;

// Bring schema into scope as module 'schema'
mod schema;
mod models {
  pub mod greeting;
}

use chrono::{DateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::figment::{
  util::map,
  value::{Map, Value},
};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{catch, catchers, get, post, launch, Request};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes, JsonSchema};
use rocket_sync_db_pools::{database, diesel as rkt_dsl};
use std::env;

#[database("my_db")]
struct DbConn(rkt_dsl::PgConnection);

// This implementation is required for routes where the db connection
// appears as a guard, and comes straight from Okapi's FAQ.
impl<'r> OpenApiFromRequest<'r> for DbConn {
  fn from_request_input(
    _gen: &mut OpenApiGenerator,
    _name: String,
    _required: bool,
  ) -> rocket_okapi::Result<RequestHeaderInput> {
    Ok(RequestHeaderInput::None)
  }
}

#[openapi]
#[get("/greetings")]
async fn get_greetings(conn: DbConn) -> Json<Vec<models::greeting::Greeting>> {
  use self::schema::greetings::dsl::*;

  Json(
    conn
      .run(|c| greetings.load::<models::greeting::Greeting>(c).expect("boom"))
      .await,
  )
}

#[openapi]
#[post("/greetings", data = "<value>")]
async fn add_greeting(conn: DbConn, value: Json<models::greeting::NewGreeting>) -> Status {
  use self::schema::greetings::dsl::*;
  conn
    .run(|c| {
      insert_into(greetings)
        .values(&value.into_inner())
        .execute(c)
        .expect("kaboom")
    })
    .await;
  rocket::http::Status::NoContent
}

#[derive(Serialize, JsonSchema)]
struct ApiError {
  statuscode: u16,
  error: String
}

#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> Json<ApiError> {
  Json(ApiError {
    statuscode: status.code,
    error: format!("{}", status.reason().unwrap_or("Internal error"))
  })
}

#[launch]
fn rocket() -> _ {
  dotenv().ok();

  // Build config map for db
  let db: Map<_, Value> = map! {
    "url" => env::var("DATABASE_URL").unwrap().into()
  };

  // Add it to the config as "my_db"
  let figment = rocket::Config::figment().merge(("databases", map!["my_db" => db]));

  // Use custom config in favor of the regular `.build()`
  rocket::custom(figment)
    .mount("/", openapi_get_routes![get_greetings, add_greeting])
    .mount(
      "/swagger",
      make_swagger_ui(&SwaggerUIConfig {
        url: "../openapi.json".to_string(),
        ..Default::default()
      }),
    )
    .register("/", catchers![default_catcher])
    .attach(DbConn::fairing())
}
