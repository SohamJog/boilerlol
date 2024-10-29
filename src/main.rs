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
        let name = &cmd[4..]; // Extract the name after "who "

        // HTML content that will navigate to the Purdue Directory and autofill the form
        let html_content = format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta http-equiv="refresh" content="0; url=https://www.purdue.edu/directory/">
                <title>Purdue Directory Search</title>
            </head>
            <body>
                <h1>Redirecting to Purdue Directory for: {}</h1>
                <form action="https://www.purdue.edu/directory/" id="searchFormBasic" method="post">
                    <fieldset>
                        <label for="basicSearchInput" id="search-label">
                            Search the <b>Purdue</b> Directory
                            <input autofocus="autofocus" id="basicSearchInput" maxlength="64" name="SearchString" tabindex="5" type="text" value="{}" />
                            <a id="glass" class="icon-search" tabindex="6"></a>
                        </label>
                        <fieldset id="basic-options" class="radio">
                            <a href="/directory/Advanced" class="toggle-advanced" tabindex="7">Advanced Search</a>
                        </fieldset>
                    </fieldset>
                    <button id="search-btn" type="submit" tabindex="8" onclick="sendGaSearch();">Search</button>
                </form>
                <script>
                    // Automatically submit the form to search for the name
                    document.getElementById('searchFormBasic').submit();
                </script>
            </body>
            </html>
            "#,
            name, name
        );

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
