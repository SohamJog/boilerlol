#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::response::Redirect;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You typed in: {}", cmd);
    let command = get_command_from_query_string(&cmd);

    let redirect_url = match command {
        "tw" => String::from("https://twitter.com"),
        _ => String::from("https://google.com"),
    };
    Redirect::to(redirect_url)
}

fn main() {
    //rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite().mount("/", routes![index, search]).launch();
}

fn get_command_from_query_string(query_string: &str) -> &str {
    if query_string.contains(' ') {
        // We need to this to know where to slice the string
        let index_of_space = query_string.find(' ').unwrap_or(0);
        return &query_string[..index_of_space];
    }
    // Otherwise, return the query string as is
    &query_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        // Test with command only
        let actual = get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        let actual = get_command_from_query_string("tw @fbOpenSource");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}
