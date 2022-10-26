use rocket::fairing::AdHoc;

mod v1;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("logs", |rocket|async {
        rocket.attach(v1::stage())
    })
}