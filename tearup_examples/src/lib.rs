use rocket::{Build, Config, Rocket};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn rocket(port: u16) -> Rocket<Build> {
    let rocket_config = Config {
        port,
        ..Config::debug_default()
    };

    rocket::build()
        .configure(rocket_config)
        .mount("/", routes![index])
}
