/*
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 * 
 * Searches boilerclasses
 */

 extern crate percent_encoding;

 use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
 
 // Used as part of the percent_encoding library
 const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
 
 pub fn construct_class_url(query: &str) -> String {
     if query == "class" {
         let class_dotcom = "https://www.boilerclasses.com/";
 
         class_dotcom.to_string()
     } else {
         // Assume the other match is "class search"
         let encoded_query = utf8_percent_encode(&query[6..], FRAGMENT).to_string();
         let class_url = format!(
             "https://www.boilerclasses.com/?q={}",
             encoded_query
         );
 
         class_url
     }
 }
 
 // TODO: Add unit tests
 