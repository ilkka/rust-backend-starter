use crate::schema::greetings;

use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(AsChangeset, Debug, Queryable, Identifiable, JsonSchema, Deserialize, Serialize)]
pub struct Greeting {
  id: i32,
  greeting: String,
  created_at: DateTime<Utc>,
}

#[derive(Deserialize, Insertable, JsonSchema)]
#[table_name = "greetings"]
pub struct NewGreeting {
  greeting: String,
}
