/// Imports
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate scoped_threadpool;

#[macro_use]
extern crate log;
extern crate env_logger;

use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use piston::window::WindowSettings;
use piston_window::*;

const ITERATIONS: f64 = 20000.0; // Level of iterations.
const ZOOM_LEVEL: f64 = 111950000.0;
const VIEW_X: f64 = 0.5191; // X view position
const VIEW_Y: f64 = 0.52694; // Y view position
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct App {
  gl: GlGraphics, // OpenGL drawing backend.
}

/// Main App
impl App {
  fn render(&mut self, args: &RenderArgs, window: &mut PistonWindow) {
    let pixel = rectangle::square(0.0, 0.0, 1.0);
    let window_width = args.height as i32;
    let window_height = args.width as i32;
    let mut color: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

    self.gl.draw(args.viewport(), |c, gl| {
      // Clear the screen with black
      clear(BLACK, gl);

      // let mut pool = scoped_threadpool::Pool::new(8);
      // let mut gl_new = &gl;
      // pool.scoped(|scope| {
      //   for y in 0..window_height {
      //     scope.execute(move || {
      //       for x in 0..window_width {
      //         let is_in_set = belongs_in_set(
      //           x as f64 / ZOOM_LEVEL - VIEW_X,
      //           y as f64 / ZOOM_LEVEL - VIEW_Y,
      //         );
      //         if is_in_set == 0.0 {
      //           // Set the color to render black
      //           rectangle(BLACK, pixel, c.transform.trans(x as f64, y as f64), gl_new);
      //         } else {
      //           color = [0.2, 0.2, 1.0, is_in_set as f32] as [f32; 4];
      //           //Set the color to render black
      //           rectangle(color, pixel, c.transform.trans(x as f64, y as f64), gl_new);
      //         }
      //       }
      //     })
      //   }
      // });

      //Mandebrot set rendering
      for x in 0..window_width {
        for y in 0..window_height {
          let is_in_set = belongs_in_set(
            x as f64 / ZOOM_LEVEL - VIEW_X,
            y as f64 / ZOOM_LEVEL - VIEW_Y,
          );
          if is_in_set == 0.0 {
            // Set the color to render black
            rectangle(BLACK, pixel, c.transform.trans(x as f64, y as f64), gl);
          } else {
            color = [0.2, 0.2, 1.0, is_in_set as f32] as [f32; 4];
            //Set the color to render black
            rectangle(color, pixel, c.transform.trans(x as f64, y as f64), gl);
          }
        }
        let progress: i16 = ((x as f32 / window_width as f32) * 100.0) as i16;
        if progress % 5 == 0 {
          error!("Progress:{}%, x={}", progress, x)
        }
      }
    });
  }

  fn update(&mut self, args: &UpdateArgs, window: &mut PistonWindow) {
    // nothing here yet
  }
}

/// Checks if a given x,y point belongs in the mandelbrot set
fn belongs_in_set(x: f64, y: f64) -> f64 {
  let (mut z, mut c) = (x, y);
  for i in 0..ITERATIONS as i16 {
    let fc = z * z - c * c + x;
    let pc = 2.0 * z * c + y;
    z = fc;
    c = pc;
    if z * c > 5.0 {
      return i as f64 / ITERATIONS;
    }
  }
  return 0.0;
}

fn main() {
  let opengl = OpenGL::V4_5;
  // Initialize logger
  env_logger::init();

  // Create a Piston window.
  let mut window: PistonWindow = WindowSettings::new("fractal", (800, 640))
    .opengl(opengl)
    .fullscreen(false)
    .vsync(true)
    .exit_on_esc(true)
    .build()
    .unwrap();

  // Create a new App
  let mut app = App {
    gl: GlGraphics::new(opengl),
  };

  // Initialize app events
  let mut events = Events::new(EventSettings::new());

  // Whilst app is running
  while let Some(e) = events.next(&mut window) {
    // Render
    if let Some(r) = e.render_args() {
      app.render(&r, &mut window);
    }
    // Update
    if let Some(u) = e.update_args() {
      app.update(&u, &mut window);
    }
  }
}
