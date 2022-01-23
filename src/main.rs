#[macro_use]
extern crate diesel;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::Queryable;
use rocket::figment::{
  util::map,
  value::{Map, Value},
};
use rocket::{get, launch};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes, JsonSchema};
use rocket_sync_db_pools::{database, diesel as rkt_dsl};

mod schema;

#[database("my_db")]
struct DbConn(rkt_dsl::PgConnection);

impl<'r> OpenApiFromRequest<'r> for DbConn {
  fn from_request_input(
    _gen: &mut OpenApiGenerator,
    _name: String,
    _required: bool
  ) -> rocket_okapi::Result<RequestHeaderInput> {
    Ok(RequestHeaderInput::None)
  }
}

#[derive(Debug, Queryable, JsonSchema, Serialize)]
struct Greeting {
  id: i32,
  greeting: String,
  created_at: DateTime<Utc>,
}

#[openapi]
#[get("/")]
async fn index(conn: DbConn) -> String {
  use self::schema::greetings::dsl::*;
  conn
    .run(|c| {
      let result = greetings.load::<Greeting>(c);
      format!("{:?}", result)
    })
    .await
}

#[openapi]
#[get("/greetings")]
async fn get_greetings(conn: DbConn) -> Json<Vec<Greeting>> {
  use self::schema::greetings::dsl::*;

  Json(conn.run(|c| greetings.load::<Greeting>(c).expect("boom")).await)
}

#[launch]
fn rocket() -> _ {
  // Build config map for db
  let db: Map<_, Value> = map! {
    "url" => "postgres://postgres:postgres@localhost:5432/postgres".into()
  };
  // Add it to the config as "my_db"
  let figment = rocket::Config::figment().merge(("databases", map!["my_db" => db]));
  // Use custom config in favor of the regular `.build()`
  rocket::custom(figment)
    .mount("/", openapi_get_routes![index, get_greetings])
    .mount(
      "/swagger",
      make_swagger_ui(&SwaggerUIConfig {
        url: "../openapi.json".to_string(),
        ..Default::default()
      }),
    )
    .attach(DbConn::fairing())
}
