//! # File I/O 1-liners
//!
//! `file::get()` and `file::put()` — read and write `Vec<u8>` with one function call.
//!
//! ```rust
//! extern crate file;
//!
//! fn example() -> file::Result<()> {
//!     let data = file::get("some_input_file.dat")?;
//!     file::put("a.out", &data)?;
//!     Ok(())
//! }
//! ```
//!
//! `file::Result` is an alias for `std::io::Result`. You can use `Result<(), Box<std::error::Error>>` in places where you don't want to expose the error type.
//!
//! ## Text file 1-liners
//!
//! `file::get_text()` and `file::put_text()` — read and write `String` with one function call.
//!
//! ```rust
//! extern crate file;
//!
//! fn example() -> file::Result<()> {
//!     let string = file::get_text("hello.txt")?;
//!     file::put("bye.txt", &string)?;
//!     Ok(())
//! }
//! ```

use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{BufReader, BufRead, Read, Write};

pub use io::Result;

/// Read a file into `Vec<u8>` from the given path.
/// The path can be a string or a `Path`.
pub fn get<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    if let Ok(meta) = file.metadata() {
        data.reserve(meta.len() as usize); // Safe to truncate, since it's only a suggestion
    }
    file.read_to_end(&mut data)?;
    Ok(data)
}

/// Creates a file at the given path with contents of `Vec<u8>` or `&[u8]`, etc.
/// Overwrites, non-atomically, if the file exists.
/// The path can be a string or a `Path`.
pub fn put<P: AsRef<Path>, Bytes: AsRef<[u8]>>(path: P, data: Bytes) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.as_ref())?;
    Ok(())
}

/// Read an UTF-8 encoded file into `String` from the given path.
/// The path can be a string or a `Path`.
pub fn get_text<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let bytes = get(path)?;
    String::from_utf8(bytes).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "file did not contain valid UTF-8")
    })
}

/// Creates a file at the given path with given text contents, encoded as UTF-8.
/// Overwrites, non-atomically, if the file exists.
/// The path can be a string or a `Path`.
pub fn put_text<P: AsRef<Path>, S: AsRef<str>>(path: P, data: S) -> io::Result<()> {
    put(path, data.as_ref().as_bytes())
}

/// Reads text lines from the file
/// Similar to Python' file('name').readlines()
pub fn readlines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let f = File::open(path)?;
    let buf = BufReader::new(f);
    Ok(buf.lines().map(|l| l.unwrap()).collect::<Vec<_>>())
}

#[test]
fn it_works() {
    let s = String::from_utf8(get(file!()).unwrap()).unwrap();
    assert!(s.contains("it_works()"));

    let mut tmp_name = std::env::temp_dir();
    tmp_name.push("tmp_file_should_not_exist");

    assert!(get(tmp_name).is_err());

    let mut tmp_name = std::env::temp_dir();
    tmp_name.push("tmp_file_created");

    let data = vec![0u8,1,2,3];
    put(&tmp_name, &data).unwrap();
    assert_eq!(data, get(&tmp_name).unwrap());
    put(&tmp_name, data).unwrap();

    std::fs::remove_file(tmp_name).ok();
}

#[test]
fn it_works_with_text() {
    let s = String::from_utf8(get(file!()).unwrap()).unwrap();
    assert!(s.contains("it_works()"));

    let mut tmp_name = std::env::temp_dir();
    tmp_name.push("hello");

    put(&tmp_name, [0x80]).unwrap();
    if let Err(e) = get_text(&tmp_name) {
        assert_eq!(e.kind(), io::ErrorKind::InvalidData);
    } else {
        panic!("Should error on invalid UTF-8")
    }

    let text = "Hello, World!";
    put_text(&tmp_name, text).unwrap();
    assert_eq!(text, get_text(&tmp_name).unwrap());

    std::fs::remove_file(tmp_name).ok();
}
