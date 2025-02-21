use core::time;
use std::thread::sleep;

use rocket_dyn_templates::{Template, tera, context};

mod view;
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    // sleep(time::Duration::from_secs(5));
    Template::render("index", context! {
        title: "Hello",
    })
}

use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("data")]
struct Db(sqlx::SqlitePool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index,view::new_snippet, view::get_snippet, view::raw_snippet])
        .attach(Template::fairing())
        .attach(Db::init())
}
