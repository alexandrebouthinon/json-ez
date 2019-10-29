<p align="center">
  <img src="https://user-images.githubusercontent.com/7868838/67627583-1df9c300-f85f-11e9-9c2e-391f16b4e1c3.png"/>
</p>
<p align="center">
  <a href="https://deps.rs/crate/json-ez/0.2.0">
    <img src="https://deps.rs/crate/json-ez/0.2.0/status.svg"/>
  </a>
  <a href="https://travis-ci.com/alexandrebouthinon/json-ez">
    <img src="https://travis-ci.com/alexandrebouthinon/json-ez.svg?branch=master"/>
  </a>
  <a href="https://codecov.io/gh/alexandrebouthinon/json-ez">
    <img src="https://codecov.io/gh/alexandrebouthinon/json-ez/branch/master/graph/badge.svg" />
  </a>
  <a href="https://crates.io/crates/json-ez">
    <img src="https://img.shields.io/crates/v/json-ez.svg"/>
  </a>
  <a href="https://docs.rs/json-ez">
    <img src="https://docs.rs/json-ez/badge.svg"/>
  </a>
  <a href="https://github.com/alexandrebouthinon/json-ez/blob/master/LICENSE">
    <img alt="undefined" src="https://img.shields.io/github/license/alexandrebouthinon/json-ez.svg?style=flat">
  </a>
</p>

## About

This crate is a wrapper around `serde` and `serde_json` crates. 
It does not aim to replace those two awesome crates, the goal is to provide another more user friendly way to deal with JSON objects in Rust.

## Installation

In your `Cargo.toml` add the following line:
```toml
[dependencies]
"json-ez" = "0.1.0"
```

## Usage

### Declaring a new JSON document and fill it with data
```rust
use json_ez::Json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut j_object = Json::new();
    let quote = "So Long, and Thanks for All the Fish!";

    // Fill the new created object
    j_object.add("key1", quote);
    j_object.add("key2", 42);
    j_object.add("key3", true);

    // Get your typed values
    let string : String = j_object.get("key1")?;
    let some_uint : u32 = j_object.get("key2")?;
    let some_boolean : bool = j_object.get("key3")?;

    // Works also with explicit casts
    let same_string = j_object.get::<String>("key1")?;

    assert_eq!(quote, &string);
    assert_eq!(quote, &same_string);
    assert_eq!(true, some_boolean);
    assert_eq!(42, some_uint);
    
    Ok(())
}
```

### Create an inline complex JSON document
```rust
use json_ez::{inline, Json};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A quite complex JSON documentation with mixed types
    let inline_json = inline!(
        "title" => "The Hitchhiker's Guide to the Galaxy",
        "novels" => vec![
            inline!(
                "title" => "The Hitchhiker's Guide to the Galaxy",
                "read" => true
            ),
            inline!(
                "title" => "The Restaurant at the End of the Universe",
                "read" => true
            ),
            inline!(
                "title" => "Life, the Universe and Everything",
                "read" => true
            ),
            inline!(
                "title" => "So Long, and Thanks for All the Fish",
                "read" => true
            ),
            inline!(
                "title" => "Mostly Harmless",
                "read" => false
            ),
            inline!(
                "title" =>  "And Another Thing...",
                "read" => false
            )
        ],
        "movie" => inline!(
            "title" => "The Hitchhiker's Guide to the Galaxy",
            "release_date" => 2005
        )
    );  

    let title : String = inline_json.get("title")?;
    let number_of_novels = inline_json.get::<Vec<Json>>("novels")?.len();
    let movie_release_date: u16 =
        inline_json.get::<Json>("movie")?.get("release_date")?;

    assert_eq!("The Hitchhiker's Guide to the Galaxy", &title);
    assert_eq!(6, number_of_novels);
    assert_eq!(2005, movie_release_date);
    
    Ok(())
}
```
