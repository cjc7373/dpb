use rocket_dyn_templates::{Template, tera, context};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Hello",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
