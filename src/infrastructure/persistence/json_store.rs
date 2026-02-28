use std::{
    fs,
    io::BufReader,
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::application::errors::StorageError;

pub struct JsonStore<T> {
    path: PathBuf,
    _marker: std::marker::PhantomData<T>,
}
