extern crate reqwest;
extern crate image;

// mod super::util;

// use self::hyper::Client;
// use self::hyper::rt::{self, Future, Stream};

const MAPBOX_TOKEN : &'static str = "pk.eyJ1IjoiYnNzZGsiLCJhIjoiY2prYW42NWFlMjZkNzNra3lnYnB6djRscCJ9.KEJKmTjzzjtKVMyxS_Y93A";

// pub fn fetch_tile(tile: super::Tile) -> impl Future<Item=image::DynamicImage, Error=FetchError> {
pub fn fetch_tile(tile: super::Tile) -> Result<image::DynamicImage, FetchError> {
    let url = format!("https://api.mapbox.com/styles/v1/mapbox/streets-v10/tiles/{}/{}/{}?access_token={}", tile.z, tile.x, tile.y, MAPBOX_TOKEN);
    fetch_png(url)
}

pub fn fetch_position(pos: super::GeoZoomPosition, img_width: u32, img_height: u32) -> Result<image::DynamicImage, FetchError> {
    let url = format!("https://api.mapbox.com/styles/v1/mapbox/streets-v10/static/{},{},{},0,0/{}x{}?access_token={}", pos.longitude, pos.latitude, pos.zoom, img_width, img_height, MAPBOX_TOKEN);
    fetch_png(url)
}

fn fetch_png(url: String) -> Result<image::DynamicImage, FetchError> {
    println!("Fetching url: {}", url);
    let mut buffer: Vec<u8> = vec![];
    let mut res = reqwest::get(&url)?;
    res.copy_to(&mut buffer)?;
    let img = image::load_from_memory_with_format(&buffer, image::ImageFormat::PNG)?;
    Ok(img)
}

pub enum FetchError {
    Http(reqwest::Error),
    Image(image::ImageError),
}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<image::ImageError> for FetchError {
    fn from(err: image::ImageError) -> FetchError {
        FetchError::Image(err)
    }
}
