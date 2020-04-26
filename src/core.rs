pub mod read;
pub mod write;
use super::err::Error;
use std::{
    path::PathBuf,
    fs::File,
    io::{Read, Write},
};
