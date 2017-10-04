mod board_view;
mod colors;
mod common;

use piston_window::*;
use piston_window::math::translate;

use catan::board::Board;
use render::board_view::{BoardViewSettings, BoardView};
use render::common::RenderView;
use render::colors::WHITE;

use find_folder;

const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;

pub fn start_application_view() {
    let mut window: PistonWindow = WindowSettings::new("Catan Agent", [800, 800])
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

    let mut board = Board::random_start();
    let mut board_view_settings = BoardViewSettings::default();
    let mut board_view = BoardView::new(&board);

    let mut events = Events::new(EventSettings::new());
    window.set_lazy(true);
    while let Some(e) = window.next() {
        if let Some(press_args) = e.press_args() {
            if press_args == Button::Keyboard(Key::NumPad0) || press_args == Button::Keyboard(Key::D0) {
                board_view_settings.set_render_text(true);
            }
        }

        if let Some(release_args) = e.release_args() {
            if release_args == Button::Keyboard(Key::NumPad0) || release_args == Button::Keyboard(Key::D0) {
                board_view_settings.set_render_text(false);
            }
        }

        if let Some(r) = e.render_args() {
            let base_translate = translate(
                [
                    window.window.size().width as f64 / 2.0,
                    window.window.size().width as f64 / 2.0,
                ],
            );

            window.draw_2d(&e, |c, g| {
                let centered_context = c.append_transform(base_translate);

                clear(WHITE, g);

                board_view.render(&board_view_settings, &centered_context, &mut glyphs, g)
            });
        }
    }
}
