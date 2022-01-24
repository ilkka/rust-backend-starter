use crate::schema::greetings;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(Deserialize, Insertable, JsonSchema)]
#[table_name = "greetings"]
pub struct NewGreeting {
  greeting: String,
}

#[derive(AsChangeset, Debug, Queryable, Identifiable, JsonSchema, Deserialize, Serialize)]
pub struct Greeting {
  id: i32,
  greeting: String,
  created_at: DateTime<Utc>,
}

pub enum Error {
  CreateError(String),
  LoadError(String)
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::CreateError(s) => write!(f, "Error creating Greeting: {}", s),
      Error::LoadError(s) => write!(f, "Error loading Greetings: {}", s)
    }
  }
}

impl Greeting {
  /// Create a new greeting.
  pub fn create(value: NewGreeting, conn: &diesel::PgConnection) -> Result<Greeting, Error> {
    use crate::schema::greetings::dsl::*;

    diesel::insert_into(greetings)
      .values(&value)
      .execute(conn)
      .and(greetings.order(id.desc()).first(conn))
      .or_else(|e| Err(Error::CreateError(e.to_string())))
  }

  /// Load all greetings.
  pub fn load(conn: &diesel::PgConnection) -> Result<Vec<Greeting>, Error> {
    use crate::schema::greetings::dsl::*;

    greetings.load(conn).or_else(|e| Err(Error::LoadError(e.to_string())))
  }
}
