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
};

#[derive(Database)]
#[database("dionysus")]
struct V1Db(sqlx::PgPool);

type Result<T, E = Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Log {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    msg: String,
}

#[post("/", data = "<entry>")]
async fn create(mut db: Connection<V1Db>, entry: Json<Log>) -> Result<Created<Json<Log>>> {
    sqlx::query("INSERT INTO logs (msg) VALUES ($1)")
        .bind(&*entry.msg)
        .execute(&mut *db)
        .await?;

    Ok(Created::new("/").body(entry))
}

#[get("/<id>")]
async fn read(mut db: Connection<V1Db>, id: i64) -> Option<Json<Log>> {
    sqlx::query("SELECT id, msg FROM logs WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *db)
        .map_ok(|r| {
            Json(Log {
                id: Some(r.get("id")),
                msg: r.get("msg"),
            })
        })
        .await
        .ok()
}

#[get("/")]
async fn list(mut db: Connection<V1Db>) -> Result<Json<Vec<Log>>> {
    Ok(Json(
        sqlx::query("SELECT id, msg FROM logs")
            .fetch(&mut *db)
            .map_ok(|l| Log {
                id: l.get("id"),
                msg: l.get("msg"),
            })
            .try_collect::<Vec<_>>()
            .await?,
    ))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("logs.v1", |rocket| async {
        rocket
            .attach(V1Db::init())
            .attach(AdHoc::try_on_ignite("V1.Logs Migrations", run_migrations))
            .mount("/v1/logs", routes![read, list, create])
    })
}
