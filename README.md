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
4. Keyword: `rb` (triggers the search engine, if this search engine is not the default)
5. URL: `http://localhost:8000/search?cmd=%s` under “Other search engines”, find your search engine, select the 3 dots menu and select “Make default”


