use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;

pub trait Writable {
    fn fmt(&self) -> String;
    fn log(&self) -> Result<String, serde_json::Error>;
}

pub fn try_open(file_path: &PathBuf) -> File {
    match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path.to_str().unwrap())
    {
        Ok(file) => file,
        Err(_) => {
            kill!("unable to create/open file {}", file_path.display());
        }
    }
}

pub fn try_deserialize<T>(data: &str) -> Result<T, serde_json::Error>
where
    T: for<'a> Deserialize<'a>,
{
    serde_json::from_str::<T>(&data)
}

pub fn try_serialize<T>(data: &T) -> Result<String, serde_json::Error>
where
    T: Serialize,
{
    serde_json::to_string::<T>(&data)
}
