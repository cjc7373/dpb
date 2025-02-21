use chrono::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::{context, Template};

use crate::Db;

#[derive(FromForm)]
pub struct Snippet<'r> {
    #[field(validate = len(..1024*1024))]
    content: &'r str,
}

#[get("/")]
pub async fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/<key>")]
pub async fn get_snippet(key: &str, mut db: Connection<Db>) -> Template {
    let res = sqlx::query!("select * from pastebin where key = ?", key)
        .fetch_one(&mut **db)
        .await
        .unwrap();

    let context = context! {
        key: key,
        content: res.content,
        created_time: res.created_time,
        length: res.length,
    };

    Template::render("snippet", &context)
}

#[get("/raw/<key>")]
pub async fn raw_snippet(key: &str, mut db: Connection<Db>) -> Option<String> {
    let res = sqlx::query!("select * from pastebin where key = ?", key)
        .fetch_one(&mut **db)
        .await
        .unwrap();

    res.content
}

#[post("/", data = "<snippet>")]
pub async fn new_snippet(
    snippet: Form<Snippet<'_>>,
    mut db: Connection<Db>,
) -> Result<Redirect, Status> {
    let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 4);
    let now = Utc::now();

    let res = sqlx::query(
        "insert into pastebin (key, content, created_time, length) values (?,?,?,?) returning key",
    )
    .bind(&random_string)
    .bind(snippet.content)
    .bind(now.timestamp())
    .bind(snippet.content.len() as u32)
    .fetch_one(&mut **db)
    .await;

    match res {
        Ok(row) => {
            let a: String = row.try_get(0).unwrap();
            println!("{:?}, {:?}", a, random_string);
        }
        Err(e) => {
            println!("{:?}", e);
            if let sqlx::Error::Database(ee) = e {
                if ee.is_unique_violation() {
                    todo!();
                } else {
                    return Err(Status::InternalServerError);
                }
            }
        }
    }

    Ok(Redirect::to(uri!(get_snippet(key = &random_string))))
}
