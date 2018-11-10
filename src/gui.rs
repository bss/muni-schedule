extern crate conrod;

use std::time::Duration;
use std::collections::HashMap;
use mapbox::{StaticMapData, StaticMap};
use route_overview::{RouteOverview, RouteData};
use nextbus::{Route, Prediction, VehicleList};
use mapbox::{OverlayItem, OverlayMarker, OverlayPath, GeoPosition};

#[derive(Clone, Debug)]
pub struct StaticApp<'a> {
    pub map_data: StaticMapData,
    pub lines: (Line<'a>, Line<'a>),
}

#[derive(Copy, Clone, Debug)]
pub struct Line<'a> {
    pub tag: &'a str,
    pub monitor_stops: &'a [&'a str],
    pub color: conrod::Color,
    pub icon: Option<conrod::image::Id>,
}

pub struct App {
    pub routes: HashMap<String, Route>,
    pub predictions: HashMap<String, Prediction>,
    pub vehicles: HashMap<String, VehicleList>,
}

impl<'a> Line<'a> {
    pub fn set_icon(&mut self, image_id: conrod::image::Id) {
        self.icon = Some(image_id);
    }
}


impl App {
    pub fn new() -> Self {
        App {
            routes: HashMap::new(),
            predictions: HashMap::new(),
            vehicles: HashMap::new(),
        }
    }

    pub fn update_route_data(&mut self, route_data: Route) {
        self.routes.insert(route_data.tag.clone(), route_data);
    }

    pub fn update_prediction_data(&mut self, prediction_data: Prediction) {
        self.predictions.insert(prediction_data.route_tag.clone(), prediction_data);
    }

    pub fn update_vehicle_data(&mut self, vehicle_data: VehicleList) {
        self.vehicles.insert(vehicle_data.route_tag.clone(), vehicle_data);
    }
}

pub fn set_widgets(ref mut ui: conrod::UiCell, ids: &Ids, static_app: &StaticApp, dynamic_app: &App) {
    use conrod::{color, widget, Widget, Sizeable, Positionable, Borderable, Colorable};

    widget::Canvas::new().border(0.0).flow_down(&[
        (ids.routes_container, widget::Canvas::new().border(0.0).length(100.0).flow_right(&[
            (ids.routes_col_1_container, widget::Canvas::new().border(0.0)),
            (ids.routes_col_2_container, widget::Canvas::new().border(0.0)),
        ])),
        (ids.map_container, widget::Canvas::new().border(0.0)),
        (ids.info_bar_container, widget::Canvas::new().border(0.0).length(20.0).color(color::DARK_CHARCOAL))
    ]).set(ids.master, ui);

    let route_data_left = route_data_for_line(&static_app.lines.0, &dynamic_app.predictions);
    RouteOverview::new(&route_data_left)
        .wh_of(ids.routes_col_1_container)
        .middle_of(ids.routes_col_1_container)
        .set(ids.routes_col_1, ui);

    let route_data_right = route_data_for_line(&static_app.lines.1, &dynamic_app.predictions);
    RouteOverview::new(&route_data_right)
        .wh_of(ids.routes_col_2_container)
        .middle_of(ids.routes_col_2_container)
        .set(ids.routes_col_2, ui);

    let mut overlay_items = Vec::new();
    overlay_items.extend(overlay_items_for_route(&static_app.lines.0, &dynamic_app.routes));
    overlay_items.extend(overlay_items_for_route(&static_app.lines.1, &dynamic_app.routes));
    overlay_items.extend(overlay_items_for_vehicles(&static_app.lines.0, &dynamic_app.vehicles));
    overlay_items.extend(overlay_items_for_vehicles(&static_app.lines.1, &dynamic_app.vehicles));

    StaticMap::new(&static_app.map_data, &overlay_items)
        .wh_of(ids.map_container)
        .middle_of(ids.map_container)
        .set(ids.map, ui);

    let oldest_fetch_maybe = dynamic_app.predictions.values().map(|v| v.seconds_since_fetch()).max();
    if let Some(oldest_fetch) = oldest_fetch_maybe {
        widget::Text::new(&format!("{} sec", oldest_fetch))
                .color(color::WHITE)
                .font_size(14)
                .center_justify()
                .middle_of(ids.info_bar_container)
                .set(ids.info_bar, ui);
    }
}



fn route_data_for_line(line: &Line, predictions: &HashMap<String, Prediction>) -> RouteData {
    let mut inbound_predictions = Vec::new();
    let mut outbound_predictions = Vec::new();
    if let Some(prediction) = predictions.get(line.tag) {
        inbound_predictions = prediction.inbound.iter().map( |val| val.duration().pretty_str()).collect();
        outbound_predictions = prediction.outbound.iter().map( |val| val.duration().pretty_str()).collect();
    }
    RouteData {
        name: line.tag.clone().to_string(),
        background_color: line.color,
        inbounds: inbound_predictions,
        outbounds: outbound_predictions,
    }
}

trait PrettyDuration {
    fn pretty_str(&self) -> String;
}

impl PrettyDuration for Duration {
    fn pretty_str(&self) -> String {
        let total_seconds = self.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        let mut out = String::new();
        if minutes > 0 {
            out.push_str(&minutes.to_string());
            out.push_str(" min ");
        }
        if seconds > 0 {
            out.push_str(&seconds.to_string());
            out.push_str(" sec ");
        }
        out
    }
}

fn overlay_items_for_route(line: &Line, routes: &HashMap<String, Route>) -> Vec<OverlayItem> {
    let mut overlay_items : Vec<OverlayItem> = vec!();
    if let Some(route) = routes.get(line.tag) {
        for section in &route.path {
            let from = GeoPosition::new(section.from.latitude, section.from.longitude);
            let to = GeoPosition::new(section.to.latitude, section.to.longitude);
            let overlay_path = OverlayPath::new(
                from,
                to,
                1.5,
                line.color.with_alpha(0.6),
            );
            overlay_items.push(OverlayItem::Path(overlay_path));
        }
    }
    overlay_items
}

trait Hueable {
    fn add_hue(self, degrees: f32) -> conrod::Color;
}

impl Hueable for conrod::Color {
    fn add_hue(self, degrees: f32) -> conrod::Color {
        match self {
            conrod::Color::Hsla(h, s, l, a) => conrod::color::hsla(h + conrod::utils::degrees(degrees), s, l, a),
            conrod::Color::Rgba(r, g, b, a) => {
                let (h, s, l) = conrod::color::rgb_to_hsl(r, g, b);
                conrod::color::hsla(h + conrod::utils::degrees(degrees), s, l, a)
            },
        }
    }
}

fn overlay_items_for_vehicles(line: &Line, vehicles: &HashMap<String, VehicleList>) -> Vec<OverlayItem> {
    let mut overlay_items : Vec<OverlayItem> = vec!();
    if let Some(vehicle_list) = vehicles.get(line.tag) {
        for vehicle in vehicle_list.vehicles.iter() {
            overlay_items.push(
                OverlayItem::Marker(
                    OverlayMarker::new(
                        GeoPosition::new(vehicle.lat, vehicle.lon),
                        line.color,
                        line.icon,
                        vehicle.secs_since_report,
                        vehicle.direction.clone(),
                    )
                )
            );
        }
    }
    overlay_items
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        master,
        routes_container,

        routes_col_1,
        routes_col_1_container,

        routes_col_2_container,
        routes_col_2,

        map_container,
        map,

        info_bar_container,
        info_bar,
    }
}
