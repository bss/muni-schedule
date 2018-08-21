#[macro_use] extern crate conrod;
#[macro_use] extern crate conrod_derive;

extern crate find_folder;
extern crate image;
extern crate rusttype;

mod event_loop;
mod mapbox;

use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use event_loop::EventLoop;
use rusttype::Font;
pub use self::mapbox::{StaticMapData, StaticMap, GeoPosition, GeoZoomPosition, OverlayItem, fetch_position, OverlayMarker};

pub struct App {
    map_data: StaticMapData,
    // left_side: MuniLine,
    // right_side: MuniLine,
}

pub struct MuniLine {
    identifier: String,
}

impl App {
    pub fn new() -> Self {
        App {
            map_data: StaticMapData::new(
                GeoZoomPosition::new(37.7593836, -122.48025849999999, 13.0),
                vec!(
                    OverlayItem::Marker(
                        OverlayMarker::new(
                            GeoPosition::new(37.75527806518829, -122.49475313829402),
                            4.0,
                            conrod::color::LIGHT_BLUE
                        )
                    ),
                    OverlayItem::Marker(
                        OverlayMarker::new(
                            GeoPosition::new(37.75004556510476, -122.48588970872652),
                            4.0,
                            conrod::color::ORANGE
                        )
                    ),
                ),
            ),
            // left_side: MuniLine,
            // right_side: MuniLine,
        }
    }

    pub fn load() {
        // My stop: 5201
        // http://webservices.nextbus.com/service/publicJSONFeed?command=predictions&a=sf-muni&r=N&s=5201
    }
}

pub fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 480;

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Canvas")
        // .with_fullscreen(Some(events_loop.get_primary_monitor()));
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    // let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    // let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    // ui.fonts.insert_from_file(font_path).unwrap();
    // ui.fonts.insert(rusttype::)
    let font_data = include_bytes!("../assets/fonts/NotoSans/NotoSans-Regular.ttf");
    let font_data_vector: Vec<u8> = font_data.iter().cloned().collect();
    let font = Font::from_bytes(font_data_vector);
    ui.fonts.insert(font.unwrap());
    

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // Instantiate the generated list of widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());

    // The image map describing each of our widget->image mappings (in our case, none).
    let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut app = App::new();
    app.map_data.load_images(&display, &mut image_map);

    // Poll events from the window.
    let mut event_loop = EventLoop::new();
    'main: loop {

        // Handle all events.
        for event in event_loop.next(&mut events_loop) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
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

        // Instantiate all widgets in the GUI.
        set_widgets(ui.set_widgets(), ids, &app);

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

fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, app: &App) {
    use conrod::{color, widget, Colorable, Sizeable, Positionable, Widget, Borderable};

    widget::Canvas::new().border(0.0).flow_down(&[
        (ids.top, widget::Canvas::new().border(0.0).length(100.0).flow_right(&[
            (ids.top_left_column, widget::Canvas::new().border(0.0).flow_right(&[
                (ids.top_left_column_left, widget::Canvas::new().border(0.0).length(100.0).color(color::BLUE.with_alpha(0.2))),
                (ids.top_left_column_right, widget::Canvas::new().border(0.0).color(color::BLUE.with_alpha(0.4))),
            ])),
            (ids.top_right_column, widget::Canvas::new().border(0.0).flow_right(&[
                (ids.top_right_column_left, widget::Canvas::new().border(0.0).length(100.0).color(color::RED)),
                (ids.top_right_column_right, widget::Canvas::new().border(0.0).color(color::RED)),
            ])),
        ])),
        (ids.bottom, widget::Canvas::new().border(0.0)),
    ]).set(ids.master, ui);

    widget::Text::new("N")
        .color(color::WHITE)
        .font_size(64)
        .center_justify()
        .middle_of(ids.top_left_column_left)
        .set(ids.top_left_column_left_text, ui);

    const DEMO_TEXT: &'static str = "Inbound\n\
            Caltrain & Ball park 3min \
            Third & ... 6min \
            Caltrain & Ball park 8min \
            Outbound \
            Ocean Beach 9min \
            Ocean Beach 18min \
            Ocean Beach 27min";

    // widget::Text::new(DEMO_TEXT)
    //     .color(color::WHITE)
    //     .font_size(32)
    //     .middle_of(ids.top_left_column_right)
    //     .set(ids.top_left_column_left_text, ui);

    const PAD: conrod::Scalar = 20.0;

    widget::Text::new(DEMO_TEXT)
            // .font_id(fonts.regular)
            .color(color::WHITE)
            .padded_w_of(ids.top_left_column_right, PAD)
            .mid_top_with_margin_on(ids.top_left_column_right, PAD)
            .left_justify()
            .line_spacing(10.0)
            .set(ids.top_left_column_right_text, ui);

    widget::Text::new("7")
        .color(color::WHITE)
        .font_size(64)
        .middle_of(ids.top_right_column_left)
        .set(ids.top_right_column_left_text, ui);

    StaticMap::new(&app.map_data)
        .wh_of(ids.bottom)
        .middle_of(ids.bottom)
        .set(ids.map, ui);
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,

        top,
        top_left_column,
        top_left_column_left,
        top_left_column_left_text,
        top_left_column_right,
        top_left_column_right_text,
        top_right_column,
        top_right_column_left,
        top_right_column_left_text,
        top_right_column_right,
        bottom,

        map,
    }
}
