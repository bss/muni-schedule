extern crate reqwest;
extern crate serde_json;

use super::{FetchError, OneOrVec, Direction};

#[derive(Clone, Debug)]
pub struct VehicleList {
    pub route_tag: String,
    pub vehicles: Vec<Vehicle>,
    pub last_time: u64,
}

#[derive(Clone, Debug)]
pub struct Vehicle {
    pub id: String,
    pub lon: f64,
    pub route_tag: Option<String>,
    pub predictable: bool,
    pub speed_km_hr: f64,
    pub direction: Direction,
    pub leading_vehicle_id: Option<String>,
    pub heading: String,
    pub lat: f64,
    pub secs_since_report: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct JsonVehicleBlob {
    vehicle: OneOrVec<JsonVehicle>,
    last_time: JsonVehicleLastTime,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct JsonVehicleLastTime {
    time: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct JsonVehicle {
    id: String,
    lon: String,
    lat: String,
    route_tag: Option<String>,
    predictable: String,
    speed_km_hr: String,
    dir_tag: Option<String>,
    leading_vehicle_id: Option<String>,
    heading: String,
    secs_since_report: String,
}

impl VehicleList {
    pub fn fetch(agency: &str, route: &str, last_fetch: u64) -> Result<Self, FetchError> {
        let url = format!("http://webservices.nextbus.com/service/publicJSONFeed?command=vehicleLocations&a={}&r={}&t={}", agency, route, last_fetch);
        println!("Fetching {}", url);
        let mut response : reqwest::Response = reqwest::get(&url)?;
        let body = response.text()?;
        match serde_json::from_str(&body) {
            Ok(val) => {
                // let vehicle_blob : JsonVehicleBlob = val;
                Ok(VehicleList::from(val)?)
            },
            Err(err) => {
                println!("Got err for url: {}", url);
                println!("Resp: {:?}", body);
                println!("Err: {:?}", err);
                Err(FetchError::from(err))
            }
        }
    }

    fn from(json_vehicle: JsonVehicleBlob) -> Result<VehicleList, FetchError> {
        let json_vehicles = json_vehicle.vehicle.into_vec();
        let route_tag = json_vehicles.iter().filter_map(|v| v.route_tag.clone() ).next().ok_or(FetchError::Other)?;
        Ok(VehicleList {
            route_tag: route_tag,
            vehicles: json_vehicles.into_iter().filter_map(|v| Vehicle::from(v).ok() ).collect(),
            last_time: json_vehicle.last_time.time.parse::<u64>()?,
        })
    }
}

impl Vehicle {
    fn from(json_vehicle: JsonVehicle) -> Result<Vehicle, FetchError> {
        let dir_tag = json_vehicle.dir_tag.clone().unwrap_or(String::new());
        let direction;
        if dir_tag.contains("__I_") {
            direction = Direction::Inbound;
        } else if dir_tag.contains("__O_") {
            direction = Direction::Outbound;
        } else {
            direction = Direction::Unknown;
        }
        Ok(Vehicle {
            id: json_vehicle.id.clone(),
            lon: json_vehicle.lon.clone().parse::<f64>()?,
            route_tag: json_vehicle.route_tag.clone(),
            predictable: json_vehicle.predictable.clone().parse::<bool>()?,
            speed_km_hr: json_vehicle.speed_km_hr.clone().parse::<f64>()?,
            direction: direction,
            leading_vehicle_id: json_vehicle.leading_vehicle_id.clone(),
            heading: json_vehicle.heading.clone(),
            lat: json_vehicle.lat.clone().parse::<f64>()?,
            secs_since_report: json_vehicle.secs_since_report.clone().parse::<u64>()?,
        })
    }
}