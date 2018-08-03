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

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // Instantiate the generated list of widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());

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
        set_widgets(ui.set_widgets(), ids);

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

fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids) {
    use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    use mapbox::{Map, Position};

    widget::Canvas::new().flow_right(&[
        (ids.left_column, widget::Canvas::new().flow_down(&[
            (ids.left_column_top, widget::Canvas::new().length(100.0).flow_right(&[
                (ids.left_column_top_left, widget::Canvas::new().length(100.0).color(color::BLUE)),
                (ids.left_column_top_right, widget::Canvas::new().color(color::GRAY)),
            ])),
            (ids.left_column_bottom, widget::Canvas::new().color(color::DARK_ORANGE)),
        ])),
        (ids.right_column, widget::Canvas::new().flow_down(&[
            (ids.right_column_top, widget::Canvas::new().length(100.0)),
            (ids.right_column_bottom, widget::Canvas::new().color(color::DARK_ORANGE).pad(20.0)),
        ])),
    ]).set(ids.master, ui);

    widget::Text::new("N")
        .color(color::WHITE)
        .font_size(64)
        .middle_of(ids.left_column_top_left)
        .set(ids.left_column_top_large_text, ui);

    Map::new(Position { latitude: 37.7593836, longitude: -122.48025849999999, zoom: 12 })
        .middle_of(ids.left_column_bottom)
        .set(ids.left_column_map, ui);
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,

        left_column,
        left_column_top,
        left_column_top_left,
        left_column_top_large_text,
        left_column_top_right,
        left_column_bottom,
        left_column_map,

        right_column,
        right_column_top,
        right_column_bottom,
    }
}
