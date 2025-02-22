use chrono::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Redirect;
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
pub async fn get_snippet(key: &str, mut db: Connection<Db>) -> Option<Template> {
    let res = sqlx::query!("select * from pastebin where key = ?", key)
        .fetch_one(&mut **db)
        .await;

    match res {
        Ok(res) => {
            let context = context! {
                key: key,
                content: res.content,
                created_time: res.created_time,
                length: res.length,
            };

            Some(Template::render("snippet", &context))
        }
        Err(e) => {
            println!("err: {:?}", e);
            None
        }
    }
}

#[get("/raw/<key>")]
pub async fn raw_snippet(key: &str, mut db: Connection<Db>) -> Option<String> {
    let res = sqlx::query!("select * from pastebin where key = ?", key)
        .fetch_one(&mut **db)
        .await;

    match res {
        Ok(res) => res.content,
        Err(e) => {
            println!("err: {:?}", e);
            None
        }
    }
}

#[post("/", data = "<snippet>")]
pub async fn new_snippet(
    snippet: Form<Snippet<'_>>,
    mut db: Connection<Db>,
) -> Result<Redirect, Status> {
    let mut random_string;
    let mut key_len = 4;
    loop {
        random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), key_len);
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

        if let Err(e) = res {
            println!("{:?}", e);
            if let sqlx::Error::Database(ee) = e {
                if ee.is_unique_violation() {
                    key_len += 1;
                    continue;
                } else {
                    return Err(Status::InternalServerError);
                }
            } else {
                return Err(Status::InternalServerError);
            }
        } else {
            break;
        }
    }

    Ok(Redirect::to(uri!(get_snippet(key = &random_string))))
}
