/*
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_docs_url(query: &str) -> String {
    if query == "docs" {
        let docs_dotcom = "https://docs.google.com/";

        docs_dotcom.to_string()
    } else {
        // Assume the other match is "docs search"
        let encoded_query = utf8_percent_encode(&query[5..], FRAGMENT).to_string();
        let docs_url = format!("https://docs.google.com/document/u/0/?q={}", encoded_query);

        docs_url
    }
}

// TODO: Add unit tests
