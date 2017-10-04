extern crate rand;
extern crate piston_window;
extern crate find_folder;

mod catan;
mod render;

use render::start_application_view;

fn main() {
    start_application_view();
}