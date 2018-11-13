extern crate reqwest;
extern crate serde_json;

mod route;
mod prediction;
mod vehicle;

use std::num::{ParseIntError, ParseFloatError};
use std::str::ParseBoolError;

pub use self::route::Route;
pub use self::prediction::Prediction;
pub use self::vehicle::VehicleList;

#[derive(Clone, Debug)]
pub enum Direction {
    Inbound,
    Outbound,
    Unknown,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum OneOrVec<T> {
    One(T),
    Vector(Vec<T>),
}

#[derive(Debug)]
pub enum FetchError {
    Http(reqwest::Error),
    Json(serde_json::Error),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    ParseBoolError(ParseBoolError),
    Other,
}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}

impl From<ParseIntError> for FetchError {
    fn from(err: ParseIntError) -> Self {
        FetchError::ParseIntError(err)
    }
}

impl From<ParseFloatError> for FetchError {
    fn from(err: ParseFloatError) -> Self {
        FetchError::ParseFloatError(err)
    }
}

impl From<ParseBoolError> for FetchError {
    fn from(err: ParseBoolError) -> Self {
        FetchError::ParseBoolError(err)
    }
}

impl<T> OneOrVec<T> {
    fn into_vec(self) -> Vec<T> {
        match self {
            OneOrVec::One(val) => vec!(val),
            OneOrVec::Vector(vec) => vec,
        }
    }
}
  
