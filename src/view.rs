use rand::distributions::{Alphanumeric, DistString};
use rocket::form::Form;
use rocket_db_pools::{sqlx, Connection};
use rocket_db_pools::sqlx::Row;
use chrono::prelude::*;
use rocket_dyn_templates::{context, Template};

use crate::Db;

#[derive(FromForm)]
pub struct Snippet<'r> {
    #[field(validate = len(..1024*1024))]
    content: &'r str,
}

#[post("/", data = "<snippet>")]
pub async fn new_snippet(snippet: Form<Snippet<'_>>, mut db: Connection<Db>) -> Template {
    let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 4);
    let now = Utc::now();

    // let res = sqlx::query_as("insert into pastebin (key, content, created_time, length) values (?,?,?,?) returning id")
    //     .bind(random_string)
    //     .bind(snippet.content)
    //     .bind(now.timestamp())
    //     .fetch_one(&mut **db)
    //     .await;

    Template::render("index", context! {
        title: "Hello",
    })
}
