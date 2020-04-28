#![feature(plugin)]

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;

use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::templates::{Template, handlebars};
use rocket_contrib::serve::StaticFiles;

use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &TemplateContext {
        title: "Reflect",
        items: vec!["Home 🏡", "New Entry➕", "View Entry👀"],
        parent: "layout",
    })
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", StaticFiles::from("static/"))
        .register(catchers![not_found])
        .attach(Template::fairing())
        .launch();
}