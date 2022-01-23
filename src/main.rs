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
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes, JsonSchema};
use rocket_sync_db_pools::{database, diesel as rkt_dsl};

mod schema;

#[database("my_db")]
struct DbConn(rkt_dsl::PgConnection);

#[derive(Debug, Queryable, JsonSchema)]
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
    .mount("/", openapi_get_routes![index])
    .mount("/swagger", make_swagger_ui(&SwaggerUIConfig {
      url: "../openapi.json".to_string(),
      ..Default::default()
    }))
    .attach(DbConn::fairing())
}
