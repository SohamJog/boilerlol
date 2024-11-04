/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod class;
pub mod docs;
pub mod drive;
pub mod github;
pub mod google;
pub mod html_embdeddings;
pub mod maps;
pub mod menus;
pub mod twitter;
pub mod brightspace;
pub mod edstem;
pub mod piazza;
pub mod push;
pub mod gradescope;

pub fn get_command_from_query_string(query_string: &str) -> &str {
    if query_string.contains(' ') {
        // We need to this to know where to slice the string
        let index_of_space = query_string.find(' ').unwrap_or(0);
        return &query_string[..index_of_space];
    }
    // Otherwise, return the query string as is
    query_string
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
