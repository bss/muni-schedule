extern crate reqwest;
extern crate serde_json;

use std::time::SystemTime;
use super::FetchError;

#[derive(Clone, Debug)]
pub struct Route {
    pub title: String,
    pub tag: String,
    pub inbound: Direction,
    pub outbound: Direction,
    pub fetched_at: SystemTime,
    pub path: Vec<PathSection>,
}

#[derive(Clone, Debug)]
pub struct Direction {
    pub title: String,
    pub name: String,
    pub tag: String,
    pub stops: Vec<Stop>,
}

#[derive(Clone, Debug)]
pub struct Stop {
    pub title: String,
    pub stop_id: String,
    pub tag: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Clone, Debug)]
pub struct PathSection {
    pub from: PathPoint,
    pub to: PathPoint,
}

#[derive(Clone, Debug)]
pub struct PathPoint {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize, Debug)]
struct JsonRouteBlob {
    route: JsonRoute,
}

#[derive(Deserialize, Debug)]
struct JsonRoute {
    title: String,
    tag: String,
    stop: Vec<JsonRouteStop>,
    direction: (JsonRouteDirection, JsonRouteDirection),
    path: Vec<JsonRoutePath>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonRouteStop {
    title: String,
    stop_id: String,
    tag: String,
    lat: String,
    lon: String,
}

#[derive(Deserialize, Debug)]
struct JsonRouteDirection {
    title: String,
    name: String,
    tag: String,
    stop: Vec<JsonRouteDirectionStopTag>,
}

#[derive(Deserialize, Debug)]
struct JsonRouteDirectionStopTag {
    tag: String,
}

#[derive(Deserialize, Debug)]
struct JsonRoutePath {
    point: Vec<JsonRoutePathPoint>,
}

#[derive(Deserialize, Debug, Clone)]
struct JsonRoutePathPoint {
    lat: String,
    lon: String,
}

impl Route {
    pub fn fetch(agency: &str, route: &str) -> Result<Self, FetchError> {
        let url = format!("http://webservices.nextbus.com/service/publicJSONFeed?command=routeConfig&a={}&r={}", agency, route);
        println!("Fetching {}", url);
        let mut response : reqwest::Response = reqwest::get(&url)?;
        let body = response.text()?;
        match serde_json::from_str(&body) {
            Ok(val) => {
                let route_blob : JsonRouteBlob = val;
                Ok(Route::from(route_blob.route))
            },
            Err(err) => {
                println!("Got err for url: {}", url);
                println!("Resp: {:?}", body);
                println!("Err: {:?}", err);
                Err(FetchError::from(err))
            }
        }
    }
}

impl From<JsonRoute> for Route {
    fn from(json_route: JsonRoute) -> Route {
        let direction_0 = &json_route.direction.0;
        let direction_1 = &json_route.direction.1;

        let inbound : Direction;
        let outbound : Direction;
        if direction_0.name == "Inbound" {
            inbound = Direction::from(&direction_0, &json_route.stop);
            outbound = Direction::from(&direction_1, &json_route.stop);
        } else {
            inbound = Direction::from(&direction_1, &json_route.stop);
            outbound = Direction::from(&direction_0, &json_route.stop);
        };

        let path : Vec<PathSection> = json_route.path.into_iter().filter_map( |rp| 
            json_route_path_to_path_secions(rp).ok()
        ).flatten().collect();

        Route {
            title: json_route.title,
            tag: json_route.tag,
            inbound: inbound,
            outbound: outbound,
            fetched_at: SystemTime::now(),
            path: path,
        }
    }
}

fn json_route_path_to_path_secions(json_route_path: JsonRoutePath) -> Result<Vec<PathSection>, FetchError> {
    let mut sections : Vec<PathSection> = vec!();
    if json_route_path.point.len() > 0 {
        for i in 0..(json_route_path.point.len()-1) {
            let from = PathPoint::from(json_route_path.point[i].clone())?;
            let to = PathPoint::from(json_route_path.point[i+1].clone())?;
            let s = PathSection::new(from, to);
            sections.push(s);
        }
    }
    Ok(sections)
}

impl Direction {
    fn from(json_direction: &JsonRouteDirection, stops: &Vec<JsonRouteStop>) -> Direction {
        Direction {
            title: json_direction.title.clone(),
            name: json_direction.name.clone(),
            tag: json_direction.tag.clone(),
            stops: json_direction.stop.iter().filter_map( |stop_tag| Stop::from(stop_tag, stops).ok() ).collect(),
        }
    }
}

impl Stop {
    fn from(stop_tag: &JsonRouteDirectionStopTag, stops: &Vec<JsonRouteStop>) -> Result<Stop, FetchError> {
        let stop : &JsonRouteStop = stops.iter().find( |s| s.tag == stop_tag.tag).ok_or(FetchError::Other)?;
        Ok(Stop {
            title: stop.title.clone(),
            stop_id: stop.stop_id.clone(),
            tag: stop.tag.clone(),
            lat: stop.lat.parse::<f64>()?,
            lon: stop.lon.parse::<f64>()?,
        })
    }
}

impl PathSection {
    pub fn new(from: PathPoint, to: PathPoint) -> Self {
        PathSection {
            from: from,
            to: to,
        }
    }
}

impl PathPoint {
    fn from(json_path_point: JsonRoutePathPoint) -> Result<Self, FetchError> {
        Ok(PathPoint {
            latitude: json_path_point.lat.parse::<f64>()?,
            longitude: json_path_point.lon.parse::<f64>()?,
        })
    }
}
