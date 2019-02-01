/// Imports
extern crate astrup;
extern crate ggez;
mod constants;

/// Use
use astrup::utils::*;
use constants::*;
use ggez::{conf::*, event::*, graphics::*, input::*, mint::Vector2, timer, *};
use nalgebra::Point2;
use std::str::FromStr;
use std::{
    fs::File,
    io::{prelude::*, BufReader, BufWriter, Write},
    path::PathBuf,
};

struct PlayingState {
    dt: std::time::Duration,
    frame_ticks: Vec<i16>,
    fractal_rendered: bool,
    fractal_buffer: Vec<u8>,
    fractal_zoom: f64,
    fractal_iterations: f64,
    fractal_center_x: f64,
    fractal_center_y: f64,
    mouse_down: bool,
    magnitude_scale: f64,
}

impl PlayingState {
    fn new() -> PlayingState {
        PlayingState {
            dt: std::time::Duration::new(0, 0),
            frame_ticks: Vec::new(),
            fractal_rendered: false,
            fractal_buffer: Vec::with_capacity((FRAC_SIZE_WIDTH * FRAC_SIZE_HEIGHT) as usize),
            fractal_zoom: ZOOM,
            fractal_iterations: ITERATIONS,
            fractal_center_x: FRACTAL_CENTER_X,
            fractal_center_y: FRACTAL_CENTER_Y,
            mouse_down: false,
            magnitude_scale: 1.0,
        }
    }
}

impl ggez::event::EventHandler for PlayingState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);

        // Holding Z for Zooming Out
        if input::keyboard::is_key_repeated(ctx) && keyboard::is_key_pressed(ctx, KeyCode::Z) {
            self.magnitude_scale = convert_order_of_magnitude(self.fractal_zoom);

            println!("Z key is held down");
            self.fractal_zoom += 0.08 * self.magnitude_scale;
            self.fractal_rendered = false;
        }

        // Holding X for Zooming In
        if input::keyboard::is_key_repeated(ctx) && keyboard::is_key_pressed(ctx, KeyCode::X) {
            self.magnitude_scale = convert_order_of_magnitude(self.fractal_zoom);

            println!("X key is held down");
            self.fractal_zoom -= 0.08 * self.magnitude_scale;
            self.fractal_rendered = false;
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, WHITE);

        self.frame_ticks = update_tick_list(&self.frame_ticks, ctx);

        render_stats("delta", ctx, self).expect("Error rendering stats");
        render_stats("time", ctx, self).expect("Error rendering stats");
        render_stats("fractal", ctx, self).expect("Error rendering stats");
        render_stats("fps", ctx, self).expect("Error rendering stats");

        let (fractal_buffer, fractal_rendered) = render_mandel(
            ctx,
            self.fractal_buffer.clone(),
            self.fractal_rendered,
            self.fractal_zoom,
            self.fractal_iterations,
            self.fractal_center_x,
            self.fractal_center_y,
        );

        self.fractal_buffer = fractal_buffer;
        self.fractal_rendered = fractal_rendered;

        present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = true;
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = false;
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        // Zoom Out
        self.magnitude_scale = convert_order_of_magnitude(self.fractal_zoom);

        if keycode == KeyCode::Z {
            println!("Z key was released");
            self.fractal_zoom += 0.08 * self.magnitude_scale;
            self.fractal_rendered = false;
        }
        // Zoom In
        if keycode == KeyCode::X {
            println!("X key was released");
            if self.fractal_zoom > 0.0 {
                self.fractal_zoom -= 0.08 * self.magnitude_scale;
                self.fractal_rendered = false;
            }
        }
        // Move View Up
        if keycode == KeyCode::Up {
            println!("Up key was released");
            self.fractal_center_y -= 0.05 * self.magnitude_scale;
            self.fractal_rendered = false;
        }
        // Move View Down
        if keycode == KeyCode::Down {
            println!("Down key was released");
            self.fractal_center_y += 0.05 * self.magnitude_scale;
            self.fractal_rendered = false;
        }
        // Move View Left
        if keycode == KeyCode::Left {
            println!("Left key was released");
            self.fractal_center_x -= 0.05 * self.magnitude_scale;
            self.fractal_rendered = false;
        }
        // Move View Right
        if keycode == KeyCode::Right {
            println!("Right key was released");
            self.fractal_center_x += 0.05 * self.magnitude_scale;
            self.fractal_rendered = false;
        }
        // Increase iterations
        if keycode == KeyCode::Equals {
            println!("Equals key was released");
            self.fractal_iterations += 100.0;
            self.fractal_rendered = false;
        }
        // Decrease iterations
        if keycode == KeyCode::Key0 {
            println!("Minus key was released");
            if self.fractal_iterations > 100.0 {
                self.fractal_iterations -= 100.0;
                self.fractal_rendered = false;
            }
        }
        // Save Coordinates
        if keycode == KeyCode::S {
            println!("S key was released, Saving Coordinates");
            save_coordinates(self, ctx);
        }
        if keycode == KeyCode::L {
            println!("L key was released, Loading Coordinates");
            let (fractal_center_x, fractal_center_y, fractal_zoom, fractal_iterations) =
                load_coordinates().unwrap();

            self.fractal_center_x = fractal_center_x;
            self.fractal_center_y = fractal_center_y;
            self.fractal_zoom = fractal_zoom;
            self.fractal_iterations = fractal_iterations;
            self.fractal_rendered = false;
        }
    }
}

fn convert_order_of_magnitude(zoom: f64) -> f64 {
    let order = order_of_magnitude(zoom);
    match order {
        0 => 1.0,
        -1 => 1.0,
        -2 => 0.1,
        -3 => 0.01,
        -4 => 0.001,
        -5 => 0.000_1,
        -6 => 0.000_01,
        -7 => 0.000_001,
        -8 => 0.000_000_1,
        -9 => 0.000_000_01,
        -10 => 0.000_000_001,
        -11 => 0.000_000_000_1,
        -12 => 0.000_000_000_01,
        -13 => 0.000_000_000_001,
        -14 => 0.000_000_000_000_1,
        -15 => 0.000_000_000_000_01,
        -16 => 0.000_000_000_000_001,
        -17 => 0.000_000_000_000_000_1,
        -18 => 0.000_000_000_000_000_01,
        -19 => 0.000_000_000_000_000_001,
        -20 => 0.000_000_000_000_000_000_1,
        _ => 0.000_000_000_000_000_000_000_000_1,
    }
}

fn update_tick_list(frame_ticks: &[i16], ctx: &mut Context) -> Vec<i16> {
    let mut ticks = frame_ticks.to_owned();

    let delta_time = timer::delta(ctx).subsec_millis() as i16;

    if ticks.len() > FPS_FRAME_COUNT {
        ticks.remove(0);
        ticks.push(delta_time);
    } else {
        ticks.push(delta_time);
    }

    ticks
}

fn save_coordinates(state: &mut PlayingState, _ctx: &mut Context) {
    let path = "coords";

    let mut output = File::create(path).unwrap();

    let mut fract_out: Vec<f64> = Vec::new();
    fract_out.push(state.fractal_center_x);
    fract_out.push(state.fractal_center_y);
    fract_out.push(state.fractal_zoom);
    fract_out.push(state.fractal_iterations);

    for value in fract_out {
        writeln!(output, "{}", value).expect("Unable to write to file");
    }
}

fn load_coordinates() -> Result<(f64, f64, f64, f64), Box<dyn std::error::Error>> {
    let input = File::open("coords")?;
    let buff_reader = BufReader::new(input);

    let mut loaded: Vec<f64> = Vec::new();

    for line in buff_reader.lines() {
        println!("{:?}", line);
        loaded.push(f64::from_str(&line?)?);
    }

    Ok((loaded[0], loaded[1], loaded[2], loaded[3]))
}

fn render_stats(
    _stat: &str,
    ctx: &mut Context,
    state: &mut PlayingState,
) -> std::result::Result<(), ggez::GameError> {
    let stat = _stat.to_string();

    let window_width = graphics::screen_coordinates(ctx).w;
    let window_height = graphics::screen_coordinates(ctx).h;

    let font = Some(Font::new(ctx, "/font/Roboto-BoldItalic.ttf").unwrap());
    let scale = Some(graphics::Scale::uniform(26.0));
    let color = Some(Color::new(0.0, 0.0, 1.0, 1.0));

    let mut text_location: Point2<f32> = Point2::new(0.0, 0.0);
    let mut stat_text: Text;

    if stat == "delta" {
        let frame_time = timer::delta(ctx).subsec_millis();

        text_location = Point2::new(window_width - 240.0, 0.0);
        stat_text = Text::new(TextFragment {
            text: format!("Last Frame: {:?}ms", frame_time),
            color,
            font,
            scale,
        });
    } else if stat == "fractal" {
        text_location = Point2::new(1.5, window_height / 2.0);

        stat_text = Text::new(TextFragment {
            text: format!(
                "Iterations: {:?}\nZoom: {:?}\nx:{:}\ny:{:}\nMagnitude:{}",
                state.fractal_iterations,
                state.fractal_zoom,
                state.fractal_center_x,
                state.fractal_center_y,
                state.magnitude_scale
            ),
            color,
            font,
            scale,
        });
    } else if stat == "time" {
        let running_time = timer::time_since_start(ctx).as_secs();

        text_location = Point2::new(window_width - 350.0, 0.0);
        stat_text = Text::new(TextFragment {
            text: format!("Time: {:?}s", running_time),
            color,
            font,
            scale,
        });
    } else if stat == "fps" {
        let mut tick_total: i16 = 0;

        for tick in state.frame_ticks.to_owned() {
            tick_total += tick
        }

        text_location = Point2::new(window_width - 80.0, 0.0);
        stat_text = Text::new(TextFragment {
            text: format!("Fps: {}", 1000 / (tick_total / FPS_FRAME_COUNT as i16)),
            color,
            font,
            scale,
        });
    } else {
        let fps = timer::fps(ctx);

        stat_text = Text::new(TextFragment {
            text: format!("Fps: {:.1}", fps),
            color,
            font,
            scale,
        });
    }

    draw(ctx, &stat_text, DrawParam::default().dest(text_location))
}

fn render_mandel(
    ctx: &mut Context,
    pixel_buffer: Vec<u8>,
    rendered: bool,
    zoom: f64,
    iterations: f64,
    center_x: f64,
    center_y: f64,
) -> (Vec<u8>, bool) {
    let mut pix_buff: Vec<u8> = pixel_buffer.clone();

    let mut rendered = rendered;

    let min_x = center_x - (zoom / 2.0);
    let min_y = center_y - (zoom / 2.0);

    if !rendered {
        pix_buff.clear();
        for y in 0..FRAC_SIZE_HEIGHT as i64 {
            for x in 0..FRAC_SIZE_WIDTH as i64 {
                let is_in_set = compute_mandel(
                    min_x + x as f64 / FRAC_SIZE_WIDTH * zoom,
                    min_y + y as f64 / FRAC_SIZE_HEIGHT * zoom,
                    iterations,
                );
                if is_in_set != 0.0 {
                    pix_buff.push(0);
                    pix_buff.push(0);
                    pix_buff.push(255);
                    pix_buff.push((is_in_set * 255.0) as u8);
                } else {
                    pix_buff.push(0);
                    pix_buff.push(0);
                    pix_buff.push(0);
                    pix_buff.push(255);
                }
            }
        }
        rendered = true
    } else {
        pix_buff = pixel_buffer.clone();
    }

    let fractal_image = Image::from_rgba8(
        ctx,
        FRAC_SIZE_WIDTH as u16,
        FRAC_SIZE_HEIGHT as u16,
        &pix_buff,
    )
    .unwrap();

    let scale: Vector2<f32> = Vector2 { x: 1.0, y: 1.6 };
    let point: Point2<f32> = Point2::new(0.0, 0.0);

    draw(
        ctx,
        &fractal_image,
        DrawParam::default().scale(scale).dest(point),
    )
    .expect("Error drawing fractal");

    (pix_buff, rendered)
}

fn compute_mandel(x: f64, y: f64, iterations: f64) -> f64 {
    let (mut z, mut c) = (x, y);
    for i in 0..iterations as i16 {
        let fc = z * z - c * c + x;
        let pc = 2.0 * z * c + y;
        z = fc;
        c = pc;
        if z * c > 5.0 {
            return f64::from(i) / iterations;
        }
    }
    0.0
}

pub fn main() {
    let state_playing = &mut PlayingState::new();

    let app_config = conf::Conf {
        window_mode: WindowMode {
            width: APP_WIDTH,
            height: APP_HEIGHT,
            borderless: false,
            fullscreen_type: FullscreenType::Desktop,
            hidpi: false,
            resizable: true,
            maximized: false,
            ..WindowMode::default()
        },
        window_setup: WindowSetup {
            title: "Test".to_string(),
            samples: NumSamples::Two,
            icon: "".to_owned(),
            vsync: true,
            srgb: true,
            transparent: false,
        },
        backend: Backend::default().gl().version(3, 2),
        modules: ModuleConf::default(),
    };

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("FractalBoi", "Terrahop")
        .add_resource_path(PathBuf::from("./assets"))
        .conf(app_config)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state_playing).expect("Something went wrong");
}
