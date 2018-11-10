extern crate reqwest;
extern crate serde_json;

use std::time::{SystemTime, Duration};
use super::{FetchError, OneOrVec};

#[derive(Clone, Debug)]
pub struct Prediction {
    pub route_tag: String,
    pub route_title: String,
    pub inbound: Vec<DirectionPrediction>,
    pub outbound: Vec<DirectionPrediction>,
    pub fetched_at: SystemTime,
}

#[derive(Clone, Debug)]
pub struct DirectionPrediction {
    pub is_departure: bool,
    pub minutes: u8,
    pub seconds: u8,
    pub trip_tag: String,
    pub vehicle: String,
    pub affected_by_layover: Option<bool>,
    pub block: String,
    pub dir_tag: String,
    pub epoch_time: u64,
    pub vehicles_in_consist: Option<u8>,
}

#[derive(Deserialize, Debug)]
struct JsonPredictionsBlob {
    predictions: OneOrVec<JsonPrediction>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonPrediction {
    agency_title: String,
    direction: OneOrVec<JsonPredictionDirection>,
    route_tag: String,
    route_title: String,
    stop_tag: String,
    stop_title: String,
}

#[derive(Deserialize, Debug, Clone)]
struct JsonPredictionDirection {
    title: String,
    prediction: OneOrVec<JsonPredictionDirectionPrediction>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct JsonPredictionDirectionPrediction {
    is_departure: String,
    minutes: String,
    seconds: String,
    trip_tag: String,
    vehicle: String,
    affected_by_layover: Option<String>,
    block: String,
    dir_tag: String,
    epoch_time: String,
    vehicles_in_consist: Option<String>,
}

impl Prediction {
    pub fn fetch(agency: &str, route: &str, stop_tags: &Vec<&str>) -> Result<Self, FetchError> {
        let mut query_params = Vec::<String>::new();
        query_params.push(String::from("command=predictionsForMultiStops"));
        query_params.push(format!("a={}", agency));
        for s in stop_tags {
            query_params.push(format!("stops={}|{}", route, s));
        } 
        let url = format!("http://webservices.nextbus.com/service/publicJSONFeed?{}", query_params.join("&"));
        println!("Fetching {}", url);
        let mut response : reqwest::Response = reqwest::get(&url)?;
        let body = response.text()?;
        match serde_json::from_str(&body) {
            Ok(val) => {
                let prediction_blob : JsonPredictionsBlob = val;
                Ok(Prediction::from(prediction_blob.predictions))
            },
            Err(err) => {
                println!("Got err for url: {}", url);
                println!("Resp: {:?}", body);
                println!("Err: {:?}", err);
                Err(FetchError::from(err))
            }
        }
    }

    fn from(json_prediction: OneOrVec<JsonPrediction>) -> Prediction {
        let json_predictions = json_prediction.into_vec();
        let route_title = json_predictions[0].route_title.clone();
        let route_tag = json_predictions[0].route_tag.clone();
        let all_prediction_directions : Vec<JsonPredictionDirection> = json_predictions
            .into_iter()
            .flat_map(|jp| jp.direction.into_vec() )
            .collect();

        Prediction {
            route_title: route_title,
            route_tag: route_tag,
            inbound: predictions_for(all_prediction_directions.clone(), "Inbound"),
            outbound: predictions_for(all_prediction_directions.clone(), "Outbound"),
            fetched_at: SystemTime::now(),
        }
    }

    pub fn seconds_since_fetch(&self) -> u64 {
        match self.fetched_at.elapsed() {
            Ok(val) => val.as_secs(),
            Err(_) => 99999,
        }
    }
}

impl DirectionPrediction {
    pub fn duration(&self) -> Duration {
        Duration::new((self.minutes as u64) * 60 + (self.seconds as u64), 0)
    }
}

fn predictions_for(jpd : Vec<JsonPredictionDirection>, direction_str: &str) -> Vec<DirectionPrediction> {
    let mut predictions : Vec<DirectionPrediction> = jpd.into_iter()
        .filter( |json| json.title.contains(direction_str) )
        .flat_map( |json| json.prediction.into_vec().into_iter().map( |jsp| DirectionPrediction::from(jsp) ) )
        .collect();
    predictions.sort_by_key( |val| val.minutes );
    predictions
}

impl From<JsonPredictionDirectionPrediction> for DirectionPrediction {
    fn from(json_direction_prediction: JsonPredictionDirectionPrediction) -> DirectionPrediction {
        DirectionPrediction {
            is_departure: json_direction_prediction.is_departure.clone().parse::<bool>().unwrap_or(false),
            minutes: json_direction_prediction.minutes.clone().parse::<u8>().unwrap_or(0),
            seconds: json_direction_prediction.seconds.clone().parse::<u8>().unwrap_or(0),
            trip_tag: json_direction_prediction.trip_tag.clone(),
            vehicle: json_direction_prediction.vehicle.clone(),
            affected_by_layover: json_direction_prediction.affected_by_layover.clone().map_or(None, |v| v.parse::<bool>().ok() ),
            block: json_direction_prediction.block.clone(),
            dir_tag: json_direction_prediction.dir_tag.clone(),
            epoch_time: json_direction_prediction.epoch_time.clone().parse::<u64>().unwrap_or(0),
            vehicles_in_consist: json_direction_prediction.affected_by_layover.clone().map_or(None, |v| v.parse::<u8>().ok() ),
        }
    }
}
