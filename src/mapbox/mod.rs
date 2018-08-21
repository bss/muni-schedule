extern crate find_folder;
extern crate image;
extern crate conrod;
extern crate glium;

mod api;
mod util;

use conrod::{Sizeable, Widget, Positionable, Rect, Range, Colorable};
use conrod::widget::{self, CommonBuilder};

const STATIC_MAP_WIDTH: u32 = 800;
const STATIC_MAP_HEIGHT: u32 = 800;

pub use self::api::{fetch_tile, fetch_position};
pub use self::util::{GeoPosition, GeoZoomPosition, Tile};

pub struct StaticMapData {
    /// Position to show
    pub position: GeoZoomPosition,

    map_background: Option<conrod::image::Id>,

    overlay_items: Vec<OverlayItem>,
}

impl StaticMapData {
    pub fn new(position: GeoZoomPosition, overlay_items: Vec<OverlayItem>) -> Self {
        StaticMapData {
            position: position,
            map_background: None,
            overlay_items: overlay_items,
        }
    }

    pub fn load_images(&mut self, display: &glium::Display, image_map: &mut conrod::image::Map<glium::texture::Texture2d>) {
        if let Ok(dynamic_tile_image) = fetch_position(self.position, STATIC_MAP_WIDTH, STATIC_MAP_HEIGHT) {
            let raw_image = match dynamic_tile_image {
                image::ImageRgb8(val) => {
                    let dimensions = val.dimensions();
                    glium::texture::RawImage2d::from_raw_rgb_reversed(&val.into_raw(), dimensions)
                },
                image::ImageRgba8(val) => {
                    let dimensions = val.dimensions();
                    glium::texture::RawImage2d::from_raw_rgba_reversed(&val.into_raw(), dimensions)
                },
                _ => panic!("Invalid image!"),
            };
            if let Ok(texture) = glium::texture::Texture2d::new(display, raw_image) {
                let image_id = image_map.insert(texture);
                self.map_background = Some(image_id);
            } else {
                panic!("couldn't create texture");
            }
        }
    }
}

pub enum OverlayItem {
    Marker(OverlayMarker)
}

pub struct OverlayMarker {
    position: GeoPosition,
    size: f64,
    color: conrod::Color,
}

impl OverlayMarker {
    pub fn new(position: GeoPosition, size: f64, color: conrod::Color) -> Self {
        OverlayMarker {
            position: position,
            size: size,
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

    pub data: &'a StaticMapData,
}

widget_ids!(
    struct Ids {
        map_image,
        circle,
        cirlc_red,
        overlay_items[]
    }
);

/// The `State` of the `Map` widget that will be cached within the `Ui`.
pub struct State {
    ids: Ids,
}

impl<'a> StaticMap<'a> {
    pub fn new(data: &'a StaticMapData) -> Self {
        StaticMap {
            common: widget::CommonBuilder::default(),
            data: data,
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
        let widget::UpdateArgs { id, state, style, rect, ui, .. } = args;
        let StaticMap { data, .. } = self;
        let StaticMapData { position, map_background, overlay_items, .. } = data;

        widget::Image::new(map_background.unwrap())
            .wh_of(id)
            .middle_of(id)
            .graphics_for(id)
            .source_rectangle(source_rect_for_image(rect))
            .set(state.ids.map_image, ui);

        if state.ids.overlay_items.len() < overlay_items.len() {
            state.update(|state| state.ids.overlay_items.resize(overlay_items.len(), &mut ui.widget_id_generator()));
        }

        let iter = state.ids.overlay_items.iter().zip(overlay_items.iter()).enumerate();
        for (i, (&item_id, item)) in iter {
            match item {
                OverlayItem::Marker(marker) => {
                    let xy_pos = rect_position_from_geo_position(position, marker.position, rect);
                    widget::Circle::fill(marker.size)
                        .color(marker.color)
                        .xy(xy_pos)
                        .set(item_id, ui);
                },
            };
        }
    }
}

fn source_rect_for_image(rect: Rect) -> Rect {
    let img_w = rect.w();
    let img_h = rect.h();
    let img_left_pad = ((STATIC_MAP_WIDTH as f64) - img_w) / 2.0;
    let img_top_pad = ((STATIC_MAP_HEIGHT as f64) - img_h) / 2.0;
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
