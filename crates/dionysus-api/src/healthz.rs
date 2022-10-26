use rocket::{fairing::AdHoc, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct HealthZSpec {
    pub version: &'static str,
}

#[get("/healthz")]
fn get() -> Json<HealthZSpec> {
    Json(HealthZSpec { version: "develop" })
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("HealthZ Stage", |rocket| async {
        rocket.mount("/", routes![get])
    })
}
