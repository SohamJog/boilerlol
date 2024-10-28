#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::response::Redirect;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You typed in {}", cmd);

    let command = utils::get_command_from_query_string(&cmd);

    let redirect_url = match command {
        "gh" => utils::github::construct_github_url(&cmd),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "drive" => utils::drive::construct_drive_url(&cmd),
        "maps" => utils::maps::construct_maps_url(&cmd),
        "docs" => utils::docs::construct_docs_url(&cmd),
        "class" => utils::class::construct_class_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd),
    };

    Redirect::to(redirect_url)
}

fn main() {
    //rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite().mount("/", routes![index, search]).launch();
}
