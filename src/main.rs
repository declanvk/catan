extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod catan;
mod render;

use render::start_application_view;

fn main() {
    start_application_view();
}