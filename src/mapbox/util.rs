use std;

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub x: i64,
    pub y: i64,
    pub z: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct GeoPosition {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoPosition {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        GeoPosition {
            latitude: latitude,
            longitude: longitude,
        }
    }

    pub fn with_zoom(self, zoom: f64) -> GeoZoomPosition {
        GeoZoomPosition::new(self.latitude, self.longitude, zoom)
    }
}


#[derive(Copy, Clone, Debug)]
pub struct GeoZoomPosition {
    pub latitude: f64,
    pub longitude: f64,
    pub zoom: f64,
}

impl GeoZoomPosition {
    pub fn new(latitude: f64, longitude: f64, zoom: f64) -> Self {
        GeoZoomPosition {
            latitude: latitude,
            longitude: longitude,
            zoom: zoom,
        }
    }

    pub fn pixel_position(&self) -> [f64; 2] {
        let lat_rad = self.latitude / 180.0 * std::f64::consts::PI;
        let n = (2.0 as f64).powf(self.zoom);
        let x = (self.longitude + 180.0) / 360.0 * n;
        let y = (1.0 - (lat_rad.tan() + (1.0 / lat_rad.cos())).ln() / std::f64::consts::PI) / 2.0 * n;

        [x * 512.0, y  * 512.0]
    }
}
