use rocket_dyn_templates::Template;

mod view;
#[macro_use]
extern crate rocket;

use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("data")]
struct Db(sqlx::SqlitePool);

#[launch]
fn rocket() -> _ {
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
}
