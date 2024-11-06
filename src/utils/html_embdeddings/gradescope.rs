use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

pub fn construct_gradescope_html(query: &str) -> Result<String, Box<dyn Error>> {
    // Strip 'gr ' from the query string
    let cmd = &query[3..];

    // Fetch the HTML of the Gradescope courses page
    let url = "https://www.gradescope.com/";
    let response = get(url)?.text()?;

    // Parse the HTML content
    let document = Html::parse_document(&response);
    println!("Document: {:?}", document);

    // Define the selector to extract course names and URLs
    let course_selector = Selector::parse(".courseBox").unwrap(); // Adjust selector if needed
    let name_selector = Selector::parse(".courseBox--name").unwrap(); // For course name
    let link_selector = Selector::parse("a").unwrap(); // To get the URL

    // Find the closest match
    let closest_match = find_closest_match(
        &document,
        cmd,
        &course_selector,
        &name_selector,
        &link_selector,
    )?;

    // Return HTML to redirect to the closest course
    Ok(format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="refresh" content="0; url={}">
            <title>Redirecting to Gradescope</title>
        </head>
        <body>
            <h1>Redirecting to course: {}</h1>
        </body>
        </html>
        "#,
        closest_match.url, closest_match.name
    ))
}

// Struct to hold course info
#[derive(Debug)]
struct Course {
    name: String,
    url: String,
}

// Function to search for the closest matching course
fn find_closest_match(
    document: &Html,
    cmd: &str,
    course_selector: &Selector,
    name_selector: &Selector,
    link_selector: &Selector,
) -> Result<Course, Box<dyn Error>> {
    let mut closest_match: Option<Course> = None;
    let mut min_distance = usize::MAX;

    // Iterate over each course element
    for course_elem in document.select(course_selector) {
        // Get the course name
        let name = course_elem
            .select(name_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>();

        // Get the link (URL)
        let link = course_elem
            .select(link_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap_or("");

        // Calculate a simple measure of "closeness" (e.g., substring match)
        let distance = calculate_distance(&name, cmd);

        // Update closest match if this is a better match
        if distance < min_distance {
            closest_match = Some(Course {
                name: name.clone(),
                url: link.to_string(),
            });
            min_distance = distance;
        }
    }

    closest_match.ok_or_else(|| "No matching course found.".into())
}

// Simple function to calculate "closeness" (you can use more advanced algorithms like Levenshtein distance)
fn calculate_distance(course_name: &str, cmd: &str) -> usize {
    course_name.to_lowercase().contains(&cmd.to_lowercase()) as usize
}
