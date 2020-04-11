# toiletdb 🚽🦀

> Flushes an object to a JSON file. Rust implementation of https://github.com/maxogden/toiletdb

A Rust key/value store backed by a JSON file.

## Usage

```rust
use toiletdb::Toiletdb;

  // pass the name of the json file to use
fn example() -> Result<(), std::io::Error> {
  let mut db = Toiletdb::new("data.json")?;

  // write some key/value pairs to data.json
  db.write("test", 123)?;
  db.write("name", "toiletdb")?;
  db.write("rust", true)?;

  // get the entire data.json contents
  let data: String = db.read()?;

  // read a value from a key
  if let Some(v) = db.read_key("test") {
     assert_eq!(v, 123);
  }

  // delete a key/value pair
  db.delete("test")?;

  // reset state and delete data.json
  db.flush()?;
  Ok(())
}
```

## API

### db.write(key, value)

sets `key` to `val` inside the JSON file

### db.read()

read the entire JSON file to a String

### db.read_key(key)

get the value of a key

### db.delete(key)

deletes `key` and it's value from the JSON file

### db.flush()

resets state and deletes the JSON file

## Installation

With [cargo-edit](https://github.com/killercup/cargo-edit):

```shell
cargo add toiletdb
```

Or manually add toiletdb to Cargo.toml

```shell
toiletdb = "0.1.0"
```

## See Also

- [`toiletdb`](https://github.com/maxogden/toiletdb)

## License

MIT
