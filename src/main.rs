#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use lazy_static::lazy_static;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod tools;

#[derive(Serialize, Clone)]
pub struct Tool {
    name: String,
    description: String,
    url: String,
}

impl Tool {
    pub fn new(name: &str, description: &str, url: &str) -> Self {
        Tool {
            name: name.to_string(),
            description: description.to_string(),
            url: url.to_string(),
        }
    }
}

lazy_static! {
    static ref ALL_TOOLS: Vec<Tool> = vec![
        Tool::new(
            "String Word Count",
            "Counts Words In A Text",
            "/tools/strings/count",
        ),
        Tool::new(
            "String Word Count",
            "Counts Words In A Text",
            "/tools/strings/count",
        )
    ];
}

#[get("/")]
fn index() -> Template {
    Template::render("index", HashMap::<String, String>::new())
}

#[get("/search/<query>")]
fn search(query: String) -> Json<Vec<Tool>> {
    let query = query.to_lowercase();
    Json(
        ALL_TOOLS
            .iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&query)
                    || t.description.to_lowercase().contains(&query)
            })
            .map(|t| t.clone())
            .collect::<Vec<Tool>>(),
    )
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static"))
        .mount(
            "/",
            routes![index, tools::strings::count::count_string, search],
        )
        .launch();
}