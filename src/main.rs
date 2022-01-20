use rocket::figment::{
  util::map,
  value::{Map, Value},
};
use rocket::{get, launch, routes};
use rocket_sync_db_pools::{diesel, database};

#[database("my_db")]
struct DbConn(diesel::PgConnection);

#[get("/")]
fn index() -> &'static str {
  "Hello world"
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
