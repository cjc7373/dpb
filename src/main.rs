use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};
use rocket_dyn_templates::Template;

mod view;
#[macro_use]
extern crate rocket;

use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("data")]
struct Db(sqlx::SqlitePool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                view::index,
                view::new_snippet,
                view::get_snippet,
                view::raw_snippet
            ],
        )
        .attach(Template::fairing())
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
}
