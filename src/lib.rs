//! Flushes an object to a JSON file. Rust implementation of https://github.com/maxogden/toiletdb
//!
//! # Examples
//!
//! ```
//! use toiletdb::Toiletdb;
//!
//!   // pass the name of the json file to use
//! fn example() -> Result<(), std::io::Error> {
//!   let mut db = Toiletdb::new("data.json")?;
//!
//!   // write some key/value pairs to data.json
//!   db.write("test", 123)?;
//!   db.write("name", "toiletdb")?;
//!   db.write("rust", true)?;
//!
//!   // get the entire data.json contents
//!   let data: String = db.read()?;
//!
//!   // read a value from a key
//!   if let Some(v) = db.read_key("test") {
//!      assert_eq!(v, 123);
//!   }
//!
//!   // delete a key/value pair
//!   db.delete("test")?;
//!
//!   // reset state and delete data.json
//!   db.flush()?;
//!   Ok(())
//! }
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::PathBuf;
use tempfile::NamedTempFile;

/// Toiletdb Struct with a file, filename, and JSON state
#[derive(Debug)]
pub struct Toiletdb {
    // file: File,
    path: PathBuf,
    state: HashMap<String, Value>,
}

impl Toiletdb {
    /// pass the name of the JSON file to use
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self, Error> {
        let path = path.into();

        Ok(Self {
            path,
            state: HashMap::new(),
        })
    }

    /// sets `key` to `val` inside the JSON file
    pub fn write<K, V>(&mut self, key: K, value: V) -> Result<(), Error>
    where
        K: Into<String>,
        V: serde::Serialize,
    {
        self.state.insert(key.into(), serde_json::to_value(value)?);
        write_file(&self.path, &self.state)?;
        Ok(())
    }

    /// read data from the JSON file
    pub fn read(&mut self) -> Result<String, Error> {
        let json = fs::read_to_string(&self.path)?;
        Ok(json)
    }

    /// read a value from a key
    pub fn read_key<K: Into<String>>(&mut self, key: K) -> Option<&Value> {
        let value = self.state.get(&key.into());
        value
    }

    /// deletes `key` from the JSON file
    pub fn delete<K: Into<String>>(&mut self, key: K) -> Result<String, Error> {
        self.state.remove(&key.into());
        write_file(&self.path, &self.state)?;
        let json = fs::read_to_string(&self.path)?;
        Ok(json)
    }

    /// resets state and deletes the JSON file
    pub fn flush(&mut self) -> Result<(), Error> {
        self.state = HashMap::new();
        fs::remove_file(&self.path)?;
        Ok(())
    }
}

// TODO: Unit test this function
/// Writes the JSON state to a tempfile first to verify that the file fits on disk.
/// If the tempfile write is successful, persist the data to the JSON file.
fn write_file<V: serde::Serialize>(path: &PathBuf, state: V) -> Result<(), Error> {
    let tmpfile = NamedTempFile::new()?;
    serde_json::to_writer_pretty(&tmpfile, &state)?;
    tmpfile.persist(&path)?;
    Ok(())
}
