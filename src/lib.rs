pub mod error;
use crate::error::Error;

//! # Json EZ
//! This crate is a wrapper around `serde` and `serde_json` crates.
//! Those two libraries are awesome when you want to serialise Rust struct
//! or deserialise JSON string into Rust string.
//!
//! However, when it comes to creating or manipulating JSON documents,
//! they often result in unnecessarily long and complex code.
//!
//! If your main goal is to simply create a JSON document,
//! for example when calling a remote JSON API, this box provides you with
//! simple wrappers like Rust macros but also implicit cast when adding
//! or getting data from you JSON object..
//!
//! ## Declaring new JSON document and fill it with data
//! ```
//! use json_ez::Json;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut j_object = Json::new();
//!     let quote = "So Long, and Thanks for All the Fish!";
//!
//!     // Fill the new created object
//!     j_object.add("key1", quote);
//!     j_object.add("key2", 42);
//!     j_object.add("key3", true);
//!
//!     // Get your typed values
//!     let string : String = j_object.get("key1")?;
//!     let some_uint : u32 = j_object.get("key2")?;
//!     let some_boolean : bool = j_object.get("key3")?;
//!
//!     // Works also with explicit casts
//!     let same_string = j_object.get::<String>("key1")?;
//!
//!     assert_eq!(quote, &string);
//!     assert_eq!(quote, &same_string);
//!     assert_eq!(true, some_boolean);
//!     assert_eq!(42, some_uint);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Create inline complex JSON document
//! ```
//! use json_ez::{inline, Json};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // A quite complex JSON documentation with mixed types
//!     let inline_json = inline!(
//!         "title" => "The Hitchhiker's Guide to the Galaxy",
//!         "novels" => vec![
//!             inline!(
//!                 "title" => "The Hitchhiker's Guide to the Galaxy",
//!                 "read" => true
//!             ),
//!             inline!(
//!                 "title" => "The Restaurant at the End of the Universe",
//!                 "read" => true
//!             ),
//!             inline!(
//!                 "title" => "Life, the Universe and Everything",
//!                 "read" => true
//!             ),
//!             inline!(
//!                 "title" => "So Long, and Thanks for All the Fish",
//!                 "read" => true
//!             ),
//!             inline!(
//!                 "title" => "Mostly Harmless",
//!                 "read" => false
//!             ),
//!             inline!(
//!                 "title" =>  "And Another Thing...",
//!                 "read" => false
//!             )
//!         ],
//!         "movie" => inline!(
//!             "title" => "The Hitchhiker's Guide to the Galaxy",
//!             "release_date" => 2005
//!         )
//!     );  
//!
//!     let title : String = inline_json.get("title")?;
//!     let number_of_novels = inline_json.get::<Vec<Json>>("novels")?.len();
//!     let movie_release_date: u16 =
//!         inline_json.get::<Json>("movie")?.get("release_date")?;
//!
//!     assert_eq!("The Hitchhiker's Guide to the Galaxy", &title);
//!     assert_eq!(6, number_of_novels);
//!     assert_eq!(2005, movie_release_date);
//!     
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::error::Error;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{
    json,
    value::{from_value, Value},
};

/// A struct offering a user friendly abstraction to JSON object.
/// Acting as a wrapper of an inner `HashMap<String, serde_json::value::Value>`
///
/// # Example
/// ```
/// use json_ez::Json;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut j_object = Json::new();
///     let quote = "So Long, and Thanks for All the Fish!";
///
///     // Fill the new created object
///     j_object.add("key1", quote);
///     j_object.add("key2", 42);
///     j_object.add("key3", true);
///
///     // Get your typed values
///     let string : String = j_object.get("key1")?;
///     let some_uint : u32 = j_object.get("key2")?;
///     let some_boolean : bool = j_object.get("key3")?;
///
///     // Works also with explicit casts
///     let same_string = j_object.get::<String>("key1")?;
///
///     assert_eq!(quote, &string);
///     assert_eq!(quote, &same_string);
///     assert_eq!(true, some_boolean);
///     assert_eq!(42, some_uint);
///     
///     Ok(())
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Json {
    #[serde(flatten)]
    json_data: HashMap<String, Value>,
}

impl Json {
    /// Simple constructor to create a new `Json` instance and
    /// initialise the inner `HashMap<String, serde_json::Value>`
    pub fn new() -> Self {
        Json {
            json_data: HashMap::new(),
        }
    }

    /// Add a new item in a `Json` instance.
    /// If the given key already exists in document,
    /// the associated value will be updated with the new one.
    pub fn add<V: Serialize>(&mut self, k: &str, v: V) {
        self.json_data.insert(k.into(), json!(v));
    }

    /// Get value associated to the given key from a `Json` instance.
    /// # Errors
    /// Return an `Err(json_ez::error::NotFound)` if the given
    /// key doesn't exists in the current `Json` instance
        let value = match self.json_data.get(k.into()) {
            Some(v) => v,
            None => {
                return Err(Error::NotFound(
                    k.into(),
                    format!("{:?}", &self.json_data),
                ))
            }
        };
        match from_value(value.clone()) {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::CannotConvert(format!("{:?}", value))),
        }
    }
}

/// Create a new `json_ez::Json` using the PHP array syntax.
/// It makes complex JSON document inline declaration easier and more readable.
///
/// # Example
/// ```
/// use json_ez::{inline, Json};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // A quite complex JSON documentation with mixed types
///     let inline_json = inline!(
///         "title" => "The Hitchhiker's Guide to the Galaxy",
///         "novels" => vec![
///             inline!(
///                 "title" => "The Hitchhiker's Guide to the Galaxy",
///                 "read" => true
///             ),
///             inline!(
///                 "title" => "The Restaurant at the End of the Universe",
///                 "read" => true
///             ),
///             inline!(
///                 "title" => "Life, the Universe and Everything",
///                 "read" => true
///             ),
///             inline!(
///                 "title" => "So Long, and Thanks for All the Fish",
///                 "read" => true
///             ),
///             inline!(
///                 "title" => "Mostly Harmless",
///                 "read" => false
///             ),
///             inline!(
///                 "title" =>  "And Another Thing...",
///                 "read" => false
///             )
///         ],
///         "movie" => inline!(
///             "title" => "The Hitchhiker's Guide to the Galaxy",
///             "release_date" => 2005
///         )
///     );  
///
///     let title : String = inline_json.get("title")?;
///     let number_of_novels = inline_json.get::<Vec<Json>>("novels")?.len();
///     let movie_release_date: u16 =
///         inline_json.get::<Json>("movie")?.get("release_date")?;
///
///     assert_eq!("The Hitchhiker's Guide to the Galaxy", &title);
///     assert_eq!(6, number_of_novels);
///     assert_eq!(2005, movie_release_date);
///     
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! inline {
    ($( $key: expr => $val: expr ),*) => {{
         use $crate::Json;
         let mut map = Json::new();
         $( map.add($key, $val); )*
         map
    }}
}

/// Deserialize an instance of `json_ez::Json` from a `String` of JSON text.
///
/// # Example
/// ```
/// use json_ez::{deserialise, Json};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let json_string = r#"{
///         "title": "The Hitchhiker's Guide to the Galaxy",
///         "type": "movie",
///         "release_date": 2005
///     }"#;  
///
///     let json_object = deserialise!(json_string)?;
///     let title : String = json_object.get("title")?;
///     let kind : String = json_object.get("type")?;
///     let release_date: u16 = json_object.get("release_date")?;
///
///     assert_eq!("The Hitchhiker's Guide to the Galaxy", &title);
///     assert_eq!("movie", &kind);
///     assert_eq!(2005, release_date);
///     
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! deserialise {
    ($item: expr) => {{
        use $crate::Json;
        serde_json::from_str::<Json>(&$item)
    }};
}

/// Serialize the given `json_ez::Json` instance as a `String` of JSON.
///
/// # Example
/// ```
/// use json_ez::{inline, Json, serialise};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let json = inline!("valid" => "json");
///     let json_string = serialise!(json)?;
///
///     assert_eq!(r#"{"valid":"json"}"#, json_string);
///     
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! serialise {
    ($item: expr) => {{
        serde_json::to_string(&$item)
    }};
}