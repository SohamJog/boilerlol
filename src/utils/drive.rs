/*
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_drive_url(query: &str) -> String {
    if query == "drive" {
        let drive_dotcom = "https://drive.google.com/";

        drive_dotcom.to_string()
    } else {
        // Assume the other match is "drive search"
        let encoded_query = utf8_percent_encode(&query[6..], FRAGMENT).to_string();
        let drive_url = format!(
            "https://drive.google.com/drive/u/0/search?q={}",
            encoded_query
        );

        drive_url
    }
}

// TODO: Add unit tests
