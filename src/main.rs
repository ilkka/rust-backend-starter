#[macro_use]
extern crate diesel;

// Bring schema into scope as module 'schema'
mod schema;
mod models {
  pub mod greeting;
}
mod logger;

use dotenv::dotenv;
use log::{error, warn, info, debug};
use rocket::figment::{
  util::map,
  value::{Map, Value},
};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{catch, catchers, get, launch, post, Request};
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
async fn get_greetings(conn: DbConn) -> Result<Json<Vec<models::greeting::Greeting>>, Status> {
  conn
    .run(|c| {
      models::greeting::Greeting::load(c)
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
async fn add_greeting(
  conn: DbConn,
  value: Json<models::greeting::NewGreeting>,
) -> Result<Json<models::greeting::Greeting>, Status> {
  conn
    .run(|c| {
      models::greeting::Greeting::create(value.into_inner(), c)
        .and_then(|g| Ok(Json(g)))
        .or_else(|e| {
          error!("{}", e);
          Err(Status::InternalServerError)
        })
    })
    .await
}

#[derive(Serialize, JsonSchema)]
struct ApiError {
  statuscode: u16,
  error: String,
}

#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> Json<ApiError> {
  Json(ApiError {
    statuscode: status.code,
    error: format!("{}", status.reason().unwrap_or("Internal error")),
  })
}

#[launch]
fn rocket() -> _ {
  dotenv().ok();
  let _logger = logger::setup_logger().expect("Could not configure logger");

  // Build config map for db
  let db: Map<_, Value> = map! {
    "url" => env::var("DATABASE_URL").unwrap().into()
  };

  // Add it to the config as "my_db"
  let figment = rocket::Config::figment().merge(("databases", map!["my_db" => db]));

  info!("Launching");
  debug!("Launched");
  warn!("Whoops!");

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
