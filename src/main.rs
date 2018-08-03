#[macro_use] extern crate conrod;

// #![deny(warnings)]
extern crate hyper;

extern crate find_folder;
extern crate image;

mod event_loop;

use hyper::Client;
use hyper::rt::{Future, Stream};
use conrod::{widget, Colorable, Positionable, Sizeable, Widget, color};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use event_loop::EventLoop;

const MAPBOX_TOKEN : &'static str = "pk.eyJ1IjoiYnNzZGsiLCJhIjoiY2prYW42NWFlMjZkNzNra3lnYnB6djRscCJ9.KEJKmTjzzjtKVMyxS_Y93A";

const TILE_WIDTH : u32 = 256;
const TILE_HEIGHT : u32 = 256;

pub fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    let tiles_count_width : u32 = ((WIDTH as f64) / (TILE_WIDTH as f64)).ceil() as u32;
    let tiles_count_height : u32 = ((HEIGHT as f64) / (TILE_HEIGHT as f64)).ceil() as u32;

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Image Widget Demonstration")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The `WidgetId` for our background and `Image` widgets.
    widget_ids!(
        struct Ids {
            background,
            tiles[]
        }
    );
    let mut ids = Ids::new(ui.widget_id_generator());

    // Create our `conrod::image::Map` which describes each of our widget->image mappings.
    // In our case we only have one image, however the macro may be used to list multiple.
    let mut image_map = conrod::image::Map::new();

    for i in 0..tiles_count_width {
        
    }

    // Poll events from the window.
    let mut event_loop = EventLoop::new();
    'main: loop {

        // Handle all events.
        for event in event_loop.next(&mut events_loop) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        {
            let ui = &mut ui.set_widgets();
            // Draw a light blue background.
            widget::Canvas::new().color(color::WHITE).set(ids.background, ui);



            ids.tiles.resize(5, &mut ui.widget_id_generator());
            for (i, &id) in ids.tiles.iter().enumerate() {
                let rust_logo = load_rust_logo(&display);
                let (w, h) = (rust_logo.get_width(), rust_logo.get_height().unwrap());
                let rust_logo_id = image_map.insert(rust_logo);
                widget::Image::new(rust_logo_id).w_h(w as f64, h as f64).x_y(-(w as f64) * (i as f64), 200.0).set(id, ui);
            }
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

// Load the Rust logo from our assets folder to use as an example image.
fn load_rust_logo(display: &glium::Display) -> glium::texture::Texture2d {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let path = assets.join("images/rust.png");
    let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
    let image_dimensions = rgba_image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}

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

struct Tile {
    x: i64,
    y: i64,
    z: u32,
}

fn get_tile_number(lat_deg: f64, lng_deg: f64, zoom: u32) -> Tile {
    let lat_rad = lat_deg / 180.0 * std::f64::consts::PI;
    let n = (2.0 as f64).powf(zoom as f64);
    let x = ((lng_deg + 180.0) / 360.0 * n) as i64;
    let y = ((1.0 - (lat_rad.tan() + (1.0 / lat_rad.cos())).ln() / std::f64::consts::PI) / 2.0 * n) as i64;
  
    Tile { x: x, y: y, z: zoom }
}
