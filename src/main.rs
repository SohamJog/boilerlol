#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::response::content;
use rocket::response::Redirect;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// TODO: cleanup with helper functions
#[get("/search?<cmd>")]
fn search(cmd: String) -> Result<Redirect, content::Html<String>> {
    println!("You typed in {}", cmd);

    if cmd.starts_with("who ") {
        let html_content = utils::html_embdeddings::directory::construct_directory_html(&cmd);
        return Err(content::Html(html_content));
    }

    // Redirect logic for other commands
    let redirect_url = match utils::get_command_from_query_string(&cmd) {
        "gh" => utils::github::construct_github_url(&cmd),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "drive" => utils::drive::construct_drive_url(&cmd),
        "maps" => utils::maps::construct_maps_url(&cmd),
        "docs" => utils::docs::construct_docs_url(&cmd),
        "class" => utils::class::construct_class_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd),
    };

    Ok(Redirect::to(redirect_url)) // Return the redirect for other commands
}

fn main() {
    //rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite().mount("/", routes![index, search]).launch();
}
