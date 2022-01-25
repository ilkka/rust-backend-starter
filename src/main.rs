#[macro_use]
extern crate diesel;

mod database;
mod schema;
mod domain;
mod logger;
mod routes;

use dotenv::dotenv;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{catch, catchers, launch, Request};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi_get_routes, JsonSchema};

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

  // Add it to the config as "my_db"
  let figment = rocket::Config::figment().merge(("databases", database::database_config()));

  // Use custom config in favor of the regular `.build()`
  rocket::custom(figment)
    .mount("/", openapi_get_routes![routes::greetings::get_greetings, routes::greetings::add_greeting])
    .mount(
      "/swagger",
      make_swagger_ui(&SwaggerUIConfig {
        url: "../openapi.json".to_string(),
        ..Default::default()
      }),
    )
    .register("/", catchers![default_catcher])
    .attach(database::DbConn::fairing())
}
