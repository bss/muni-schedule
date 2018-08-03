// #![deny(warnings)]
extern crate hyper;
extern crate image;

use hyper::Client;
use hyper::rt::{Future, Stream};

const MAPBOX_TOKEN : &'static str = "pk.eyJ1IjoiYnNzZGsiLCJhIjoiY2prYW42NWFlMjZkNzNra3lnYnB6djRscCJ9.KEJKmTjzzjtKVMyxS_Y93A";

fn fetch_tile(tile: Tile) -> impl Future<Item=image::DynamicImage, Error=FetchError> {
    let url = format!("https://api.mapbox.com/styles/v1/mapbox/streets-v10/tiles/{}/{}/{}?access_token={}", tile.z, tile.x, tile.y, MAPBOX_TOKEN);
    let uri = url.parse::<hyper::Uri>().unwrap();
    fetch_png(uri)
}

fn fetch_png(url: hyper::Uri) -> impl Future<Item=image::DynamicImage, Error=FetchError> {
    let client = Client::new();

    client
        .get(url)
        .and_then(|res| {
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            let img = image::load_from_memory_with_format(&body.into_bytes(), image::ImageFormat::PNG)?;
            Ok(img)
        })
        .from_err()
}

enum FetchError {
    Http(hyper::Error),
    Image(image::ImageError),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<image::ImageError> for FetchError {
    fn from(err: image::ImageError) -> FetchError {
        FetchError::Image(err)
    }
}
