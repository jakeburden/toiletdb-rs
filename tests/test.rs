use serde_json::Value;
use std::io::Error;
use tempfile::NamedTempFile;
use toiletdb::Toiletdb;

#[test]
fn writes_and_reads() -> Result<(), Error> {
    let file = NamedTempFile::new()?;
    let mut db = Toiletdb::new(file.path())?;
    db.write("test", 123)?;
    db.write("test-2", "second test")?;
    let data = db.read()?;
    let v: Value = serde_json::from_str(&data)?;
    assert_eq!(v["test"], 123);
    assert_eq!(v["test-2"], "second test");

    if let Some(v) = db.read_key("test") {
        assert_eq!(v, 123);
    }

    Ok(())
}

#[test]
fn deletes() -> Result<(), Error> {
    let file = NamedTempFile::new()?;
    let mut db = Toiletdb::new(file.path())?;
    db.write("test", 123)?;
    db.write("test-2", "second test")?;
    db.delete("test-2")?;
    let data = db.read()?;
    let v: Value = serde_json::from_str(&data)?;
    assert_eq!(v["test"], 123);
    assert_eq!(v["test-2"], Value::Null);
    Ok(())
}

#[test]
fn flushes() -> Result<(), Error> {
    let file = NamedTempFile::new()?;
    let mut db = Toiletdb::new(file.path())?;
    db.flush()?;
    let exists = file.path().exists();
    assert_eq!(exists, false);
    Ok(())
}
