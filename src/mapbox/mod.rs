extern crate find_folder;
extern crate image;

mod util;

use conrod::{Colorable, Positionable, Sizeable, Widget, color};
use conrod::widget::{self, Style, CommonBuilder};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;

const TILE_WIDTH : u32 = 256;
const TILE_HEIGHT : u32 = 256;

pub use self::util::{Position, Tile};

/// A widget for displaying a mapbox map
#[derive(Copy, Clone, Debug, WidgetCommon)]
pub struct Map {
    /// Builder parameters that are common to all `Widget`s.
    #[conrod(common_builder)]
    pub common: CommonBuilder,
    /// Position to show
    pub position: Position,
}

widget_ids!(
    struct Ids {
        tiles[]
    }
);

/// The `State` of the `Map` widget that will be cached within the `Ui`.
pub struct State {
    ids: Ids,
}

impl Map {
    pub fn new(position: Position) -> Self {
        Map {
            common: widget::CommonBuilder::default(),
            position: position,
        }
    }
}

impl Widget for Map {
    type State = State;
    type Style = Style;
    type Event = ();

    fn init_state(&self, _: widget::id::Generator) -> Self::State {
        State {
            src_rect: None,
            image_id: self.image_id,
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    fn default_x_dimension(&self, ui: &Ui) -> Dimension {
        match self.src_rect.as_ref() {
            Some(rect) => Dimension::Absolute(rect.w()),
            None => widget::default_x_dimension(self, ui),
        }
    }

    fn default_y_dimension(&self, ui: &Ui) -> Dimension {
        match self.src_rect.as_ref() {
            Some(rect) => Dimension::Absolute(rect.h()),
            None => widget::default_y_dimension(self, ui),
        }
    }

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { state, .. } = args;
        let Map { image_id, src_rect, .. } = self;

        if state.image_id != image_id {
            state.update(|state| state.image_id = image_id);
        }
        if state.src_rect != src_rect {
            state.update(|state| state.src_rect = src_rect);
        }
    }

}


// pub fn main() {
//     const WIDTH: u32 = 800;
//     const HEIGHT: u32 = 600;
//     let tiles_count_width : u32 = ((WIDTH as f64) / (TILE_WIDTH as f64)).ceil() as u32;
//     let tiles_count_height : u32 = ((HEIGHT as f64) / (TILE_HEIGHT as f64)).ceil() as u32;

//     // Build the window.
//     let mut events_loop = glium::glutin::EventsLoop::new();
//     let window = glium::glutin::WindowBuilder::new()
//         .with_title("Image Widget Demonstration")
//         .with_dimensions((WIDTH, HEIGHT).into());
//     let context = glium::glutin::ContextBuilder::new()
//         .with_vsync(true)
//         .with_multisampling(4);
//     let display = glium::Display::new(window, context, &events_loop).unwrap();

//     // construct our `Ui`.
//     let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

//     // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
//     // for drawing to the glium `Surface`.
//     let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

//     // The `WidgetId` for our background and `Image` widgets.

//     let mut ids = Ids::new(ui.widget_id_generator());

//     // Create our `conrod::image::Map` which describes each of our widget->image mappings.
//     // In our case we only have one image, however the macro may be used to list multiple.
//     let mut image_map = conrod::image::Map::new();

//     for i in 0..tiles_count_width {
        
//     }

//     // Poll events from the window.
//     let mut event_loop = EventLoop::new();
//     'main: loop {

//         // Handle all events.
//         for event in event_loop.next(&mut events_loop) {

//             // Use the `winit` backend feature to convert the winit event to a conrod one.
//             if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
//                 ui.handle_event(event);
//             }

//             match event {
//                 glium::glutin::Event::WindowEvent { event, .. } => match event {
//                     // Break from the loop upon `Escape`.
//                     glium::glutin::WindowEvent::CloseRequested |
//                     glium::glutin::WindowEvent::KeyboardInput {
//                         input: glium::glutin::KeyboardInput {
//                             virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
//                             ..
//                         },
//                         ..
//                     } => break 'main,
//                     _ => (),
//                 },
//                 _ => (),
//             }
//         }

//         {
//             let ui = &mut ui.set_widgets();
//             // Draw a light blue background.
//             widget::Canvas::new().color(color::WHITE).set(ids.background, ui);

//             ids.tiles.resize(5, &mut ui.widget_id_generator());
//             for (i, &id) in ids.tiles.iter().enumerate() {
//                 let rust_logo = load_rust_logo(&display);
//                 let (w, h) = (rust_logo.get_width(), rust_logo.get_height().unwrap());
//                 let rust_logo_id = image_map.insert(rust_logo);
//                 widget::Image::new(rust_logo_id).w_h(w as f64, h as f64).x_y(-(w as f64) * (i as f64), 200.0).set(id, ui);
//             }
//         }

//         // Render the `Ui` and then display it on the screen.
//         if let Some(primitives) = ui.draw_if_changed() {
//             renderer.fill(&display, primitives, &image_map);
//             let mut target = display.draw();
//             target.clear_color(0.0, 0.0, 0.0, 1.0);
//             renderer.draw(&display, &mut target, &image_map).unwrap();
//             target.finish().unwrap();
//         }
//     }
// }

// // Load the Rust logo from our assets folder to use as an example image.
// fn load_rust_logo(display: &glium::Display) -> glium::texture::Texture2d {
//     let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
//     let path = assets.join("images/rust.png");
//     let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
//     let image_dimensions = rgba_image.dimensions();
//     let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
//     let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
//     texture
// }
