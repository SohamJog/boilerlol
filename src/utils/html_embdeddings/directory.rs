/*
* Helper function for embedding html to search in Purdue Directory
*/


pub fn construct_directory_html(query: &str) -> String {
    // HTML content that will navigate to the Purdue Directory and autofill the form
    let name = &query[4..];
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
    html_content
}
