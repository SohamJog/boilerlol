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
    if cmd.starts_with("zz ") {
        // Call the function and handle the Result
        match utils::html_embdeddings::gradescope::construct_gradescope_html(&cmd[3..]) {
            Ok(html_content) => {
                // If the result is Ok, return the HTML content
                return Err(content::Html(html_content));
            }
            Err(e) => {
                // If an error occurs, log the error or handle it in another way
                eprintln!("Error constructing gradescope HTML: {}", e);
                return Err(content::Html(format!("<h1>Error: {}</h1>", e)));
            }
        }
    }

    // Redirect logic for other commands
    // TODO: [cleanup] Sort alphabetically
    let redirect_url = match utils::get_command_from_query_string(&cmd) {
        "gh" => utils::github::construct_github_url(&cmd),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "drive" => utils::drive::construct_drive_url(&cmd),
        "maps" => utils::maps::construct_maps_url(&cmd),
        "docs" => utils::docs::construct_docs_url(&cmd),
        "class" => utils::class::construct_class_url(&cmd),
        // TODO: [cleanup] Merge the next 4 lines
        "menus" => utils::menus::construct_menus_url(),
        "food" => utils::menus::construct_menus_url(),
        "eats" => utils::menus::construct_menus_url(),
        "noms" => utils::menus::construct_menus_url(),
        // Gradescope
        "gradescope" => utils::gradescope::construct_gradescope_url(),
        "gr" => utils::gradescope::construct_gradescope_url(),
        "gscope" => utils::gradescope::construct_gradescope_url(),
        // Brightspace
        "bs" => utils::brightspace::construct_brightspace_url(),
        "brightspace" => utils::brightspace::construct_brightspace_url(),
        "bspace" => utils::brightspace::construct_brightspace_url(),
        // Piazza
        "piazza" => utils::piazza::construct_piazza_url(),
        "pizza" => utils::piazza::construct_piazza_url(),
        "pz" => utils::piazza::construct_piazza_url(),
        // PUSH
        "push" => utils::push::construct_push_url(),
        // Edstem
        "ed" => utils::edstem::construct_edstem_url(),
        "edstem" => utils::edstem::construct_edstem_url(),
        // Default
        _ => utils::google::construct_google_search_url(&cmd),
    };

    Ok(Redirect::to(redirect_url)) // Return the redirect for other commands
}

fn main() {
    //rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite().mount("/", routes![index, search]).launch();
}
