#[macro_use]
extern crate diesel;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::insert_into;
use rocket::figment::{
  util::map,
  value::{Map, Value},
};
use rocket::http::Status;
use rocket::{get, post, launch};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes, JsonSchema};
use rocket_sync_db_pools::{database, diesel as rkt_dsl};
use self::schema::greetings; // this is needed for the table_name attribute to work

mod schema; // TODO: what exactly does this do?

#[database("my_db")]
struct DbConn(rkt_dsl::PgConnection);

// This implementation is required for routes where the db connection
// appears as a guard, and comes straight from Okapi's FAQ.
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

#[derive(Deserialize, Insertable, JsonSchema)]
#[table_name = "greetings"]
struct NewGreeting {
  greeting: String
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

#[openapi]
#[post("/greetings", data = "<value>")]
async fn add_greeting(conn: DbConn, value: Json<NewGreeting>) -> Status {
  use self::schema::greetings::dsl::*;
  conn.run(|c| insert_into(greetings).values(&value.into_inner()).execute(c).expect("kaboom")).await;
  rocket::http::Status::NoContent
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
    .mount("/", openapi_get_routes![index, get_greetings, add_greeting])
    .mount(
      "/swagger",
      make_swagger_ui(&SwaggerUIConfig {
        url: "../openapi.json".to_string(),
        ..Default::default()
      }),
    )
    .attach(DbConn::fairing())
}
