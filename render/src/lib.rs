extern crate piston_window;
extern crate catan_core;
extern crate find_folder;
#[macro_use]
extern crate log;

mod board_view;
mod colors;
mod common;

use piston_window::*;

use catan_core::board::Board;
use ::board_view::{BoardController, BoardViewSettings};
use ::common::{Controller, Renderer, Builder};
use ::colors::{BACKGROUND_SEA_BLUE, WHITE};

const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;

pub fn start_application_view() {
    let mut window: PistonWindow = WindowSettings::new("Catan Agent", [1280, 800])
        .opengl(OPEN_GL_VERSION)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let texture_settings = TextureSettings::new();
    let mut glyphs = Glyphs::new(font, factory, texture_settings).unwrap();

    let mut board = Board::balanced_start();
    let mut board_controller = BoardController::new(false, true, true, false);
    let board_view_settings = BoardViewSettings::new([0.0, 0.0], 800.0, 800.0);
    let mut board_view = board_view_settings.build();

    window.set_lazy(true);
    while let Some(e) = window.next() {
        board_controller.handle_events(&e, &mut board, &mut board_view);

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear(WHITE, g);

                board_controller.render(&board, &mut board_view, &c, &mut glyphs, g)
            });
        }
    }
}
