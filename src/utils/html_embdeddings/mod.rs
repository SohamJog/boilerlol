/*
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

 pub mod directory;


 pub fn get_html_from_query_string(query_string: &str) -> &str {
  if query_string.contains(' ') {
      // We need to this to know where to slice the string
      let index_of_space = query_string.find(' ').unwrap_or(0);
      return &query_string[..index_of_space];
  }
  // Otherwise, return the query string as is
  query_string
}