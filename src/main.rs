#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "hello Rocket APP"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
