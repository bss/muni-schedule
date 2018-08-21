
#[derive(Deserialize, Debug)]
struct Route {
    title: String,
    tag: String,
    direction_a: RouteDirection,
    direction_b: RouteDirection,
}

#[derive(Deserialize, Debug)]
struct RouteDirection {
    stops: Vec<RouteStop>,
    title: String,
    name: String,
    tag: String,
}

#[derive(Deserialize)]
struct RouteJsonBlob {
    route: Route,
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

const URL_TEMPLATE = "http://webservices.nextbus.com/service/publicJSONFeed?command=routeConfig&a={}&r={}"

pub fn fetch_route(agency: String, route: String) -> Result<Route, FetchError> {
    let url = format!(URL_TEMPLATE, agency, route);

}

fn fetch_png(url: String) -> Result<image::DynamicImage, FetchError> {
    println!("Fetching url: {}", url);
    let mut buffer: Vec<u8> = vec![];
    let mut res = reqwest::get(&url)?.json()?;
    res.copy_to(&mut buffer)?;
    let img = image::load_from_memory_with_format(&buffer, image::ImageFormat::PNG)?;
    Ok(img)
}
