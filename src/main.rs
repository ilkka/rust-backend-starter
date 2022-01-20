use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
  "Hello world"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
