extern crate image;
extern crate conrod;
extern crate glium;

mod api;
mod vehicle_icon;
mod util;

use conrod::{Sizeable, Widget, Positionable, Rect, Range, Colorable};
use conrod::widget::{self, CommonBuilder};

pub use self::api::{fetch_position, FetchError};
pub use self::util::{GeoPosition, GeoZoomPosition, Tile};
pub use super::nextbus::Direction;

#[derive(Copy, Clone, Debug)]
pub struct StaticMapData {
    pub position: GeoZoomPosition,
    pub map_background: Option<StaticMapDataBackgroundImage>,
}

#[derive(Copy, Clone, Debug)]
pub struct StaticMapDataBackgroundImage {
    pub id: conrod::image::Id,
    pub width: u32,
    pub height: u32,
}

impl StaticMapData {
    pub fn new(position: GeoZoomPosition) -> Self {
        StaticMapData {
            position: position,
            map_background: None,
        }
    }

    pub fn fetch_map_background(&self, width: u32, height: u32) -> Result<image::DynamicImage, FetchError> {
        fetch_position(self.position, width, height)
    }
}

impl StaticMapDataBackgroundImage {
    pub fn new(id: conrod::image::Id, width: u32, height: u32) -> Self {
        StaticMapDataBackgroundImage {
            id: id,
            width: width,
            height: height,
        }
    }
}

pub enum OverlayItem {
    Marker(OverlayMarker),
    Path(OverlayPath),
}

impl OverlayItem {
    pub fn marker_or_none(&self) -> Option<&OverlayMarker> {
        match self {
            OverlayItem::Marker(val) => Some(&val),
            _ => None,
        }
    }

    pub fn path_or_none(&self) -> Option<&OverlayPath> {
        match self {
            OverlayItem::Path(val) => Some(&val),
            _ => None,
        }
    }
}

pub struct OverlayMarker {
    position: GeoPosition,
    color: conrod::Color,
    icon: Option<conrod::image::Id>,
    secs_since_report: u64,
    direction: Direction,
}

impl OverlayMarker {
    pub fn new(position: GeoPosition, color: conrod::Color, icon: Option<conrod::image::Id>, secs_since_report: u64, direction: Direction) -> Self {
        OverlayMarker {
            position: position,
            color: color,
            icon: icon,
            secs_since_report: secs_since_report,
            direction: direction,
        }
    }
}

pub struct OverlayPath {
    path_start: GeoPosition,
    path_end: GeoPosition,
    width: f64,
    color: conrod::Color,
}

impl OverlayPath {
    pub fn new(path_start: GeoPosition, path_end: GeoPosition, width: f64, color: conrod::Color) -> Self {
        OverlayPath {
            path_start: path_start,
            path_end: path_end,
            width: width,
            color: color,
        }
    }
}

/// A widget for displaying a mapbox map
#[derive(WidgetCommon)]
pub struct StaticMap<'a> {
    /// Builder parameters that are common to all `Widget`s.
    #[conrod(common_builder)]
    pub common: CommonBuilder,

    pub static_data: &'a StaticMapData,
    pub overlay_items: &'a Vec<OverlayItem>,
}

widget_ids!(
    struct Ids {
        map_image,
        circle,
        cirlc_red,
        vehicles[],
        paths[],
    }
);

/// The `State` of the `Map` widget that will be cached within the `Ui`.
pub struct State {
    ids: Ids,
}

impl<'a> StaticMap<'a> {
    pub fn new(static_data: &'a StaticMapData, overlay_items: &'a Vec<OverlayItem>) -> Self {
        StaticMap {
            common: widget::CommonBuilder::default(),
            static_data: static_data,
            overlay_items: overlay_items,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
}

impl<'a> Widget for StaticMap<'a> {
    type State = State;
    type Style = Style;
    type Event = ();

    fn init_state(&self, id_generator: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_generator),
        }
    }

    fn style(&self) -> Self::Style {
        Style::default()
    }

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, ui, .. } = args;
        let StaticMap { static_data, overlay_items, .. } = self;
        let StaticMapData { position, map_background, .. } = static_data;

        if let Some(map_bg) = map_background {
            widget::Image::new(map_bg.id)
                .wh_of(id)
                .middle_of(id)
                .graphics_for(id)
                .source_rectangle(source_rect_for_image(map_bg, rect))
                .set(state.ids.map_image, ui);
        }
        
        let overlay_markers : Vec<&OverlayMarker> = overlay_items.iter().filter_map(|it| it.marker_or_none() ).collect();
        let overlay_paths : Vec<&OverlayPath> = overlay_items.iter().filter_map(|it| it.path_or_none() ).collect();

        if state.ids.vehicles.len() < overlay_markers.len() {
            state.update(|state| state.ids.vehicles.resize(overlay_markers.len(), &mut ui.widget_id_generator()));
        }
        if state.ids.paths.len() < overlay_paths.len() {
            state.update(|state| state.ids.paths.resize(overlay_paths.len(), &mut ui.widget_id_generator()));
        }

        let iter = state.ids.paths.iter().zip(overlay_paths.iter()).enumerate();
        for (_i, (&item_id, path)) in iter {
            let xy_pos_start = rect_position_from_geo_position(position, path.path_start, rect);
            let xy_pos_end = rect_position_from_geo_position(position, path.path_end, rect);
            widget::Line::abs( xy_pos_start, xy_pos_end)
                .color(path.color)
                .thickness(path.width)
                .set(item_id, ui);
        }

        let iter = state.ids.vehicles.iter().zip(overlay_markers.iter()).enumerate();
        for (_i, (&item_id, marker)) in iter {
            let xy_pos = rect_position_from_geo_position(position, marker.position, rect);
            vehicle_icon::VehicleIcon::new(marker).w_h(58.0, 58.0).xy(xy_pos).set(item_id, ui);
        }
    }
}

fn source_rect_for_image(background_image: &StaticMapDataBackgroundImage, rect: Rect) -> Rect {
    let img_w = rect.w();
    let img_h = rect.h();
    let img_left_pad = ((background_image.width as f64) - img_w) / 2.0;
    let img_top_pad = ((background_image.height as f64) - img_h) / 2.0;
    Rect {
        x: Range::new(img_left_pad, img_left_pad + img_w),
        y: Range::new(img_top_pad, img_top_pad + img_h),
    }
}

fn rect_position_from_geo_position(center_position: &GeoZoomPosition, position: GeoPosition, rect: Rect) -> conrod::position::Point {
    let center_pixels = center_position.pixel_position();
    let position_pixels = position.with_zoom(center_position.zoom).pixel_position();
    let diff = [position_pixels[0] - center_pixels[0], -(position_pixels[1] - center_pixels[1])];
    let center_point = rect.xy();
    [center_point[0] + diff[0], center_point[1] + diff[1]]
}
