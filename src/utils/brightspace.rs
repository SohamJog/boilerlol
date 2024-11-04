/*
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub fn construct_brightspace_url() -> String {
    let brightspace_dotcom = "https://purdue.brightspace.com/";
    brightspace_dotcom.to_string()
}

/*
* TODO: Add more features like searching for courses directly. 
* This can be achieved in the following ways:
* 1. Store user IDs and and subject IDs <- will need to revamp architecture for that
* 2. Somehow reverse engineer the brightspace home-page html and have some fancy implementation in html_embeddings
*/
