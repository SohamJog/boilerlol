/*
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_maps_url(query: &str) -> String {
    if query == "maps" {
        let maps_dotcom = "https://maps.google.com/";

        maps_dotcom.to_string()
    } else {
        // Assume the other match is "maps search"
        let encoded_query = utf8_percent_encode(&query[5..], FRAGMENT).to_string();
        let maps_url = format!("https://www.google.com/maps/search/{}", encoded_query);

        maps_url
    }
}

// TODO: Add unit tests
