extern crate reqwest;
extern crate serde_json;

mod route;
mod prediction;
mod vehicle;

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

impl<T> OneOrVec<T> {
    fn into_vec(self) -> Vec<T> {
        match self {
            OneOrVec::One(val) => vec!(val),
            OneOrVec::Vector(vec) => vec,
        }
    }
}
  
