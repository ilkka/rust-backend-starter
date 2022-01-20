#[macro_use]
extern crate diesel;

use rocket::figment::{
  util::map,
  value::{Map, Value},
};
use rocket::{get, launch, routes};
use rocket_sync_db_pools::{diesel as rkt_dsl, database};
use diesel::prelude::*;
use diesel::Queryable;

mod schema;

#[database("my_db")]
struct DbConn(rkt_dsl::PgConnection);

#[derive(Debug, Queryable)]
struct Greeting {
  id: i32,
  greeting: String
}

#[get("/")]
async fn index(conn: DbConn) -> String {
  use self::schema::greetings::dsl::*;
  conn.run(|c| {
    let result = greetings.load::<Greeting>(c);
    format!("{:?}", result)
  }).await
}

#[launch]
fn rocket() -> _ {
  // Build config map for db
  let db : Map<_, Value> = map! {
    "url" => "postgres://postgres:postgres@localhost:5432/postgres".into()
  };
  // Add it to the config as "my_db"
  let figment = rocket::Config::figment().merge(("databases", map!["my_db" => db]));
  // Use custom config in favor of the regular `.build()`
  rocket::custom(figment).mount("/", routes![index]).attach(DbConn::fairing())
}
