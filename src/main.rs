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

// #[get("/who?<name>")]
// fn who_search(name: String) -> content::Html<String> {
//     let html_content = format!(
//         r#"
//         <!DOCTYPE html>
//         <html lang="en">
//         <head>
//             <meta charset="UTF-8">
//             <title>Purdue Directory Search</title>
//         </head>
//         <body>
//             <p>Searching for: {}</p>
//             <iframe id="directoryFrame" src="https://www.purdue.edu/directory/" style="width:100%; height:100vh;"></iframe>

//             <script>
//                 // Wait until the iframe has loaded
//                 document.getElementById('directoryFrame').onload = function() {{
//                     var iframeDoc = document.getElementById('directoryFrame').contentWindow.document;
//                     // Fill in the search input (Assuming it has an id or name attribute, update as needed)
//                     var searchInput = iframeDoc.querySelector('input[name=\"search\"]'); // Update selector based on HTML
//                     if (searchInput) {{
//                         searchInput.value = "{}";
//                         searchInput.form.submit(); // Submit the form if the input exists
//                     }}
//                 }};
//             </script>
//         </body>
//         </html>
//         "#,
//         name, name
//     );

//     content::Html(html_content)
// }

// #[get("/search?<cmd>")]
// fn search(cmd: String) -> Redirect {
//     println!("You typed in {}", cmd);

//     let command = utils::get_command_from_query_string(&cmd);

//     let redirect_url = match command {
//         "gh" => utils::github::construct_github_url(&cmd),
//         "tw" => utils::twitter::construct_twitter_url(&cmd),
//         "drive" => utils::drive::construct_drive_url(&cmd),
//         "maps" => utils::maps::construct_maps_url(&cmd),
//         "docs" => utils::docs::construct_docs_url(&cmd),
//         "class" => utils::class::construct_class_url(&cmd),
//         _ => utils::google::construct_google_search_url(&cmd),
//     };

//     Redirect::to(redirect_url)
// }

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
