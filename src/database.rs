use rocket_sync_db_pools::{database, diesel};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket::figment::{
  util::map,
  value::{Map, Value},
};
use std::env;

#[database("my_db")]
pub struct DbConn(diesel::PgConnection);

/// This implementation is required for routes where the db connection
/// appears as a guard, and comes straight from Okapi's FAQ.
impl<'r> OpenApiFromRequest<'r> for DbConn {
  fn from_request_input(
    _gen: &mut OpenApiGenerator,
    _name: String,
    _required: bool,
  ) -> rocket_okapi::Result<RequestHeaderInput> {
    Ok(RequestHeaderInput::None)
  }
}

/// Database configuration
pub fn database_config() -> Value {
  // Build config map for db
  let db: Map<_, Value> = map! {
    "url" => env::var("DATABASE_URL").unwrap().into()
  };

  // Add it to the config as "my_db"
  Value::from(map!["my_db" => db])
}
