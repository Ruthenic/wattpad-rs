# wattpad-rs
Unofficial async Rust wrapper around the (various) Wattpad API(s)

### Documentation
See [docs.rs](https://docs.rs/wattpad)

### Examples
Stories:
```rust
use wattpad::Wattpad;

#[tokio::main]
async fn main() {
    let watt = Wattpad::new()
    .await
    .unwrap();
    let story = watt
        .get_story("336149308")
        .await
    .unwrap();

    println!("{}", story.title)
}
```
Searches:
```rust
use wattpad::{SearchSort, SearchType, Wattpad};

#[tokio::main]
async fn main() {
    let watt = Wattpad::new()
    .await
    .unwrap();

    // Text searches
    let text_search = watt
        .search(
            "bendy x reader",
            SearchType::Text,
            SearchSort::Hot,
            30,
        )
        .await
        .unwrap();

    let text_results = text_search.page(0).await.unwrap();

    println!("{}", text_results[0].title)


    // Tag searches
    let tag_search = watt
        .search(
            "bendyxreader,batim",
            SearchType::Text,
            SearchSort::Hot,
            30,
        )
        .await
        .unwrap();

    let tag_results = tag_search.page(0).await.unwrap();

    println!("{}", tag_results[0].title)
}
```