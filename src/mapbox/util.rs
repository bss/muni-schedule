use std;

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    x: i64,
    y: i64,
    z: u32,
}

pub fn get_tile_number(lat_deg: f64, lng_deg: f64, zoom: u32) -> Tile {
    let lat_rad = lat_deg / 180.0 * std::f64::consts::PI;
    let n = (2.0 as f64).powf(zoom as f64);
    let x = ((lng_deg + 180.0) / 360.0 * n) as i64;
    let y = ((1.0 - (lat_rad.tan() + (1.0 / lat_rad.cos())).ln() / std::f64::consts::PI) / 2.0 * n) as i64;
  
    Tile { x: x, y: y, z: zoom }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    latitude: f64,
    longitude: f64,
    zoom: f64,
}