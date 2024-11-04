# Purdue Bunnylol

**A Purdue-specific implementation of Meta's Bunnylol shortcut tool, featuring quick commands for common university resources and other frequently visited websites.**

While interning at Meta, I extensively used the internal tool Bunnylol. I only realized how helpful it was after my internship ended and I no longer had access! This inspired me to create a similar tool in Rust, complete with commands I frequently use. It now includes Purdue-specific shortcuts, so my friends here can also benefit from it.

> **Note:** Currently, this project isn’t hosted. Please refer to the setup section below to host it locally.

---

## Commands

Below is a list of commands included in this tool. Feel free to contribute to the repository to add more or fork it to create your own!

| Shortcut     | Args   | Website                                 | Description                          |
|--------------|--------|-----------------------------------------|--------------------------------------|
| `gh`         | `user` | GitHub                                  | Searches Github for <user>           |
| `tw`         | `query`| Twitter                                 | Searches Twitter for <query>         |
| `drive`      | `cmd`  | Google Drive                            | Searches Google Drive for <cmd>      |
| `maps`       | `cmd`  | Google Maps                             | Searches Google Maps for <cmd>       |
| `docs`       | `cmd`  | Google Docs                             | Searches Google Docs for <cmd>       |
| `class`      | `cmd`  | Boilerclasses                           | Searches boilerclasses for <cmd>     |
| `menus`      |        | Purdue Food Menus                       | Links to Purdue dining menus         |
| `food`       |        | Purdue Food Menus                       | Links to Purdue dining menus         |
| `eats`       |        | Purdue Food Menus                       | Links to Purdue dining menus         |
| `noms`       |        | Purdue Food Menus                       | Links to Purdue dining menus         |
| `gradescope` |        | Gradescope                              | Links to Gradescope                  |
| `gr`         |        | Gradescope                              | Links to Gradescope                  |
| `gscope`     |        | Gradescope                              | Links to Gradescope                  |
| `bs`         |        | Brightspace                             | Links to Brightspace                 |
| `brightspace`|        | Brightspace                             | Links to Brightspace                 |
| `bspace`     |        | Brightspace                             | Links to Brightspace                 |
| `piazza`     |        | Piazza                                  | Links to Piazza                      |
| `pizza`      |        | Piazza                                  | Links to Piazza                      |
| `pz`         |        | Piazza                                  | Links to Piazza                      |
| `push`       |        | Purdue PUSH                             | Links to PUSH                        |
| `ed`         |        | Edstem                                  | Links to Edstem                      |
| `edstem`     |        | Edstem                                  | Links to Edstem                      |
| `who`        | `cmd`  | Purdue Directory                        | Searches Purdue directory            |
| _default_    | `cmd`  | Google                                  | Constructs a Google search URL       |

---

## Contributing

This project is open for contributions. If you have useful shortcuts or improvements in mind, please feel free to open a PR or fork the repository.



# Setup

Change rust to nightly
```bash
rustup default nightly
```

You can always switch it back to default:

```bash
rustup default nightly
```


```rust
cargo run
```

## Testing on Chrome
1. Navigate to chrome://settings/searchEngines
2. Click “Add” under “Default Search Engines” and use the following values:
3, Search Engine: Rusty Bunny Local
4. Keyword: `rb` (triggers the search engine, if this search engine is not the default. Note that you can use any string here)
5. URL: `http://localhost:8000/search?cmd=%s`
6. (Optional, but highly recommended) Under “Other search engines”, find your search engine, select the 3 dots menu and select “Make default”


## Acknowledgements

This implementation is based on the tutorial [Build a Smart Bookmarking Tool with Rust and Rocket](https://developers.facebook.com/blog/post/2020/06/03/build-smart-bookmarking-tool-rust-rocket/) by Facebook Developers.


