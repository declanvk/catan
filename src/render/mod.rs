use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::color::TRANSPARENT;
use catan::board::Board;
use graphics::{Context, Graphics, Transformed, line};
use graphics::polygon::Polygon;
use graphics::math::{Vec2d, Scalar, translate, scale, rotate_radians, identity};
use catan::board::{InternalCoord, InternalTileType};
use graphics::color::{WHITE, BLACK};

use rand::Rng;
use rand;

const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;

pub struct ApplicationView {
    gl: GlGraphics,
}

impl ApplicationView {
    fn render(&mut self, args: &RenderArgs, board: &Board) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        let resource_hex = Polygon::new(RED);
        let building_hex = Polygon::new(BLUE);

        let base_translate = translate([args.width as f64 / 2.0, args.height as f64 / 2.0]);

        self.gl.draw(args.viewport(), |c, gl| {
            let centered_context = c.append_transform(base_translate);

            clear(WHITE, gl);

            for (&position, tile_type) in board.tiles.iter() {
                match tile_type {
                    &InternalTileType::BuildingTile => {
                        draw_hexagon(&building_hex, position, &centered_context, gl);
                    }
                    &InternalTileType::ResourceTile(_) => {
                        draw_hexagon(&resource_hex, position, &centered_context, gl);
                    }
                }
            }


        });
    }
}

pub fn start_application_view() {
    let mut window: Window = WindowSettings::new("catan agent", [800, 800])
        .opengl(OPEN_GL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = ApplicationView { gl: GlGraphics::new(OPEN_GL_VERSION) };

    let mut board = Board::random_start();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &board);
        }
    }
}

const RADIUS: Scalar = 0.1;
const HEXAGON_REDUCTION: Scalar = 0.85;

fn draw_hexagon<G: Graphics>(poly: &Polygon, coord: InternalCoord, context: &Context, g: &mut G) {
    let center = convert_cube_coord_to_cartesian(coord, RADIUS);
    let vertices = hexagon_vertices(center, RADIUS * HEXAGON_REDUCTION);
    poly.draw(&vertices, &context.draw_state, identity(), g);
}

fn convert_cube_coord_to_cartesian(coord: InternalCoord, size: Scalar) -> Vec2d {
    [
        size * (3.0 as f64).sqrt() * (coord.x as f64 + (coord.y as f64 / 2.0)),
        size * 1.5 * coord.y as f64,
    ]
}

fn hexagon_vertices(center: Vec2d, size: Scalar) -> [Vec2d; 6] {
    [
        [
            center[0] + size * ((60 * 0 + 30) as Scalar).to_radians().cos(),
            center[1] + size * ((60 * 0 + 30) as Scalar).to_radians().sin(),
        ],
        [
            center[0] + size * ((60 * 1 + 30) as Scalar).to_radians().cos(),
            center[1] + size * ((60 * 1 + 30) as Scalar).to_radians().sin(),
        ],
        [
            center[0] + size * ((60 * 2 + 30) as Scalar).to_radians().cos(),
            center[1] + size * ((60 * 2 + 30) as Scalar).to_radians().sin(),
        ],
        [
            center[0] + size * ((60 * 3 + 30) as Scalar).to_radians().cos(),
            center[1] + size * ((60 * 3 + 30) as Scalar).to_radians().sin(),
        ],
        [
            center[0] + size * ((60 * 4 + 30) as Scalar).to_radians().cos(),
            center[1] + size * ((60 * 4 + 30) as Scalar).to_radians().sin(),
        ],
        [
            center[0] + size * ((60 * 5 + 30) as Scalar).to_radians().cos(),
            center[1] + size * ((60 * 5 + 30) as Scalar).to_radians().sin(),
        ],
    ]
}