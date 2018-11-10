#[macro_use] extern crate serde_derive;
#[macro_use] extern crate conrod;
#[macro_use] extern crate conrod_derive;

extern crate serde;
extern crate image;
extern crate rusttype;

mod nextbus;
mod mapbox;
mod route_overview;
mod gui;

use std::time::Duration;
use std::thread::sleep;

use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use rusttype::Font;

const WIN_W: u32 = 800;
const WIN_H: u32 = 480;

enum GuiInputEvent {
    ConrodInput(conrod::event::Input),
    RouteData(nextbus::Route),
    PredictionData(nextbus::Prediction),
    VehicleData(nextbus::VehicleList),
}

pub fn main() {
    let mut static_app = gui::StaticApp {
        map_data: mapbox::StaticMapData::new(
            mapbox::GeoZoomPosition::new(37.7593836, -122.48025849999999, 13.0),
        ),
        lines: (
            gui::Line { tag: "N", monitor_stops: &["5201", "5202"], color: conrod::color::LIGHT_BLUE, icon: None},
            gui::Line { tag: "7", monitor_stops: &["3427", "3443"], color: conrod::color::LIGHT_RED, icon: None},
        ),
    };

    let mut events_loop = glium::glutin::EventsLoop::new();
    let mut winit = Winit::build(&events_loop);

    let (event_tx, event_rx) = std::sync::mpsc::channel();
    let (render_tx, render_rx) = std::sync::mpsc::channel();
    
    let events_loop_proxy = events_loop.create_proxy();

    match static_app.map_data.fetch_map_background() {
        Ok(val) => static_app.map_data.map_background = winit.load_image_into_image_map(val).ok(),
        Err(_) => panic!("Error!")
    };
    
    let tram_icon = winit.load_image(include_bytes!("../assets/images/tram_light.png"));
    static_app.lines.0.set_icon(tram_icon.unwrap());
    let bus_icon = winit.load_image(include_bytes!("../assets/images/bus_light.png"));
    static_app.lines.1.set_icon(bus_icon.unwrap());

    start_route_fetchers(static_app.lines.0.clone(), &event_tx);
    start_route_fetchers(static_app.lines.1.clone(), &event_tx);
    
    (|tx| std::thread::spawn(move || redraw_thread(tx)) )(event_tx.clone());

    std::thread::spawn(move || run_conrod(&static_app, event_rx, render_tx, events_loop_proxy));
    
    winit.run_loop(&mut events_loop, event_tx, render_rx);
}

fn redraw_thread(event_tx: std::sync::mpsc::Sender<GuiInputEvent>) {
    loop {
        event_tx.send(GuiInputEvent::ConrodInput(conrod::event::Input::Redraw)).unwrap();
        sleep(Duration::new(1, 0));
    }
}

fn start_route_fetchers(line: gui::Line<'static>, event_tx: &std::sync::mpsc::Sender<GuiInputEvent>) {
    (|tx| { std::thread::spawn(move || route_fetcher_loop(line.tag, tx)) })(event_tx.clone());
    (|tx| { std::thread::spawn(move || prediction_fetcher_loop(line.tag, &line.monitor_stops.to_vec(), tx)) })(event_tx.clone());
    (|tx| { std::thread::spawn(move || vehicle_fetcher_loop(line.tag, tx)) })(event_tx.clone());
}

fn route_fetcher_loop(line: &str, event_tx: std::sync::mpsc::Sender<GuiInputEvent>) {
    loop {
        let route = nextbus::Route::fetch("sf-muni", &line);
        match route {
            Ok(val) => event_tx.send(GuiInputEvent::RouteData(val)).unwrap(),
            Err(err) => println!("Error fetching route: {:?}", err)
        };
        sleep(Duration::new(15*60, 0));
    }
}

fn prediction_fetcher_loop(line_tag: &str, monitor_stops: &Vec<&str>, event_tx: std::sync::mpsc::Sender<GuiInputEvent>) {
    loop {
        let prediction = nextbus::Prediction::fetch("sf-muni", &line_tag, monitor_stops);
        match prediction {
            Ok(val) => event_tx.send(GuiInputEvent::PredictionData(val)).unwrap(),
            Err(err) => println!("Error fetching prediction: {:?}", err)
        };
        sleep(Duration::new(30, 0));
    }
}

fn vehicle_fetcher_loop(line: &str, event_tx: std::sync::mpsc::Sender<GuiInputEvent>) {
    loop {
        let route = nextbus::VehicleList::fetch("sf-muni", &line, 0);
        match route {
            Ok(val) => event_tx.send(GuiInputEvent::VehicleData(val)).unwrap(),
            Err(err) => println!("Error fetching route: {:?}", err)
        };
        sleep(Duration::new(15, 0));
    }
}

struct Winit {
    display: glium::Display,
    renderer: conrod::backend::glium::Renderer,
    image_map: conrod::image::Map<glium::texture::Texture2d>
}

#[derive(Debug)]
enum ImageLoadingError {
    Image(image::ImageError),
    Texture(glium::texture::TextureCreationError),
}

impl From<image::ImageError> for ImageLoadingError {
    fn from(err: image::ImageError) -> ImageLoadingError {
        ImageLoadingError::Image(err)
    }
}

impl From<glium::texture::TextureCreationError> for ImageLoadingError {
    fn from(err: glium::texture::TextureCreationError) -> ImageLoadingError {
        ImageLoadingError::Texture(err)
    }
}

impl Winit {
    pub fn build(events_loop: &glium::glutin::EventsLoop) -> Self {
        // Build the window.
        let window = glium::glutin::WindowBuilder::new()
            .with_title("Muni Schedule")
            .with_dimensions((WIN_W, WIN_H).into());
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, events_loop).unwrap();
        let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
        Winit {
            display,
            renderer,
            image_map
        }
    }

    pub fn load_image_into_image_map(&mut self, dynamic_image: image::DynamicImage) -> Result<conrod::image::Id, ImageLoadingError> {
        let raw_image = match dynamic_image {
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
        let texture = glium::texture::Texture2d::new(&self.display, raw_image)?;
        Ok(self.image_map.insert(texture))
    }

    pub fn load_image(&mut self, image_buffer: &[u8]) -> Result<conrod::image::Id, ImageLoadingError> {
        let rgba_image = image::load_from_memory(image_buffer)?.to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
        let bus_img = glium::texture::Texture2d::new(&self.display, raw_image)?;
        Ok(self.image_map.insert(bus_img))
    }

    pub fn run_loop(
        &mut self,
        events_loop: &mut glium::glutin::EventsLoop,
        event_tx: std::sync::mpsc::Sender<GuiInputEvent>,
        render_rx: std::sync::mpsc::Receiver<conrod::render::OwnedPrimitives>,
    ) {
        let mut last_update = std::time::Instant::now();
        let mut closed = false;
        while !closed {
            // We don't want to loop any faster than 60 FPS, so wait until it has been at least
            // 16ms since the last yield.
            let sixteen_ms = std::time::Duration::from_millis(16);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }

            events_loop.run_forever(|event| {
                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &self.display) {
                    event_tx.send(GuiInputEvent::ConrodInput(event)).unwrap();
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
                        } => {
                            closed = true;
                            return glium::glutin::ControlFlow::Break;
                        },
                        // We must re-draw on `Resized`, as the event loops become blocked during
                        // resize on macOS.
                        glium::glutin::WindowEvent::Resized(..) => {
                            if let Some(primitives) = render_rx.iter().next() {
                                self.draw(&primitives);
                            }
                        },
                        _ => {},
                    },
                    glium::glutin::Event::Awakened => return glium::glutin::ControlFlow::Break,
                    _ => (),
                }

                glium::glutin::ControlFlow::Continue
            });

            // Draw the most recently received `conrod::render::Primitives` sent from the `Ui`.
            if let Some(primitives) = render_rx.try_iter().last() {
                self.draw(&primitives);
            }

            last_update = std::time::Instant::now();
        }
    }

    // Draws the given `primitives` to the given `Display`.
    fn draw(&mut self, primitives: &conrod::render::OwnedPrimitives) {
        self.renderer.fill(&self.display, primitives.walk(), &self.image_map);
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
        target.finish().unwrap();
    }
}

// Conrod loop
fn run_conrod(static_app: &gui::StaticApp,
                event_rx: std::sync::mpsc::Receiver<GuiInputEvent>,
                render_tx: std::sync::mpsc::Sender<conrod::render::OwnedPrimitives>,
                events_loop_proxy: glium::glutin::EventsLoopProxy)
{
    // Construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let font_data = include_bytes!("../assets/fonts/NotoSans/NotoSans-Regular.ttf");
    let font_data_vector: Vec<u8> = font_data.iter().cloned().collect();
    let font = Font::from_bytes(font_data_vector);
    ui.fonts.insert(font.unwrap());

    // The `widget::Id` of each widget instantiated in `support::gui`.
    let ids = gui::Ids::new(ui.widget_id_generator());

    let mut app = gui::App::new();

    // Many widgets require another frame to finish drawing after clicks or hovers, so we
    // insert an update into the conrod loop using this `bool` after each event.
    let mut needs_update = true;
    'conrod: loop {
        // Collect any pending events.
        let mut events = Vec::new();
        while let Ok(event) = event_rx.try_recv() {
            events.push(event);
        }

        // If there are no events pending, wait for them.
        if events.is_empty() || !needs_update {
            match event_rx.recv() {
                Ok(event) => events.push(event),
                Err(_) => break 'conrod,
            };
        }

        needs_update = false;

        // Input each event into the `Ui`.
        for event in events {
            match event {
                GuiInputEvent::ConrodInput(val) => ui.handle_event(val),
                GuiInputEvent::RouteData(data) => app.update_route_data(data),
                GuiInputEvent::PredictionData(data) => app.update_prediction_data(data),
                GuiInputEvent::VehicleData(data) => app.update_vehicle_data(data),
            };
            needs_update = true;
        }

        gui::set_widgets(ui.set_widgets(), &ids, &static_app, &app);

        // Render the `Ui` to a list of primitives that we can send to the main thread for
        // display. Wakeup `winit` for rendering.
        if let Some(primitives) = ui.draw_if_changed() {
            if render_tx.send(primitives.owned()).is_err()
            || events_loop_proxy.wakeup().is_err() {
                break 'conrod;
            }
        }
    }
}
