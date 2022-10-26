#[macro_use]
extern crate rocket;

mod healthz;
mod library;
mod logs;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(healthz::stage())
        .attach(logs::stage())
        .attach(library::stage())
}
