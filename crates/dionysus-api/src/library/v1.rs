use {
    log::error,
    rocket::{
        fairing::{self, AdHoc},
        futures::{TryFutureExt, TryStreamExt},
        response::{status::Created, Debug},
        serde::{json::Json, Deserialize, Serialize},
        Build, Rocket,
    },
    rocket_db_pools::{
        sqlx::{self, Row},
        Connection, Database,
    },
    std::path::PathBuf,
};

#[derive(Debug, Deserialize, Serialize)]
struct Library {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    name: String,
    base_path: PathBuf,
}

#[derive(Database)]
#[database("dionysus")]
struct V1Db(sqlx::PgPool);

type Result<T, E = Debug<sqlx::Error>> = std::result::Result<T, E>;

#[get("/")]
async fn list(mut db: Connection<V1Db>) -> Result<Json<Vec<Library>>> {
    Ok(Json(
        sqlx::query("SELECT id, name, base_path FROM library")
            .fetch(&mut *db)
            .map_ok(|l| Library {
                id: l.get("id"),
                name: l.get("name"),
                base_path: PathBuf::from("."),
            })
            .try_collect::<Vec<_>>()
            .await?,
    ))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("library.v1", |rocket| async {
        rocket.attach(V1Db::init()).mount("/v1/logs", routes![list])
    })
}
