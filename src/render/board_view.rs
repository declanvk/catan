use piston_window::*;
use piston_window::character::CharacterCache;
use piston_window::math::{Scalar, Vec2d};
use piston_window::types::{Color, FontSize};

use catan::board::{InternalCoord, Board, InternalTileType, ResourceTileType};
use render::colors::*;
use render::common::{Renderer, Controller, Builder};

use std::fmt;
use std::default::Default;

pub struct BoardController {
    render_coordinate_text: bool,
    render_roll_tokens: bool,
}

impl BoardController {
    pub fn new(render_coordinate_text: bool, render_roll_tokens: bool) -> BoardController {
        BoardController {
            render_coordinate_text,
            render_roll_tokens,
        }
    }
}

impl Controller for BoardController {
    type Model = Board;
    type View = BoardView;

    fn handle_events(&mut self, e: &Event, model: &mut Board, view: &mut BoardView) {

        e.press(|button| if button == Button::Keyboard(Key::NumPad1) ||
            button == Button::Keyboard(Key::D1)
        {
            if !self.render_coordinate_text && self.render_roll_tokens {
                self.render_roll_tokens = false;
            }
            self.render_coordinate_text = !self.render_coordinate_text;
        } else if button == Button::Keyboard(Key::NumPad2) || button == Button::Keyboard(Key::D2) {
            if !self.render_roll_tokens && self.render_coordinate_text {
                self.render_coordinate_text = false;
            }
            self.render_roll_tokens = !self.render_roll_tokens;
        });

    }
}

pub struct BoardView {
    upper_left_anchor: Vec2d,
    origin: Vec2d,
    scale_width: Scalar,
    scale_height: Scalar,
    building_tile: Polygon,
    desert_tile: Polygon,
    mountain_tile: Polygon,
    hill_tile: Polygon,
    pasture_tile: Polygon,
    fields_tile: Polygon,
    forest_tile: Polygon,
    coordinate_text: Text,
    roll_token_text: Text,
    hexagon_nominal_size: Scalar,
    hexagon_actual_size: Scalar,
}

pub struct BoardViewSettings {
    upper_left_anchor: Vec2d,
    origin: Vec2d,
    scale_width: Scalar,
    scale_height: Scalar,
    building_tile_color: Color,
    desert_tile_color: Color,
    mountain_tile_color: Color,
    hill_tile_color: Color,
    pasture_tile_color: Color,
    fields_tile_color: Color,
    forest_tile_color: Color,
    coordinate_text_color: Color,
    coordinate_text_font_size: FontSize,
    coordinate_text_round: bool,
    roll_token_text_font_size: FontSize,
    roll_token_text_color: Color,
    roll_token_text_round: bool,
    hexagon_nominal_size: Scalar,
    hexagon_actual_size: Scalar,
}

impl Builder for BoardViewSettings {
    type Output = BoardView;

    fn build(&self) -> BoardView {
        BoardView {
            upper_left_anchor: self.upper_left_anchor,
            origin: self.origin,
            scale_width: self.scale_width,
            scale_height: self.scale_height,
            building_tile: Polygon::new(self.building_tile_color),
            desert_tile: Polygon::new(self.desert_tile_color),
            mountain_tile: Polygon::new(self.mountain_tile_color),
            hill_tile: Polygon::new(self.hill_tile_color),
            pasture_tile: Polygon::new(self.pasture_tile_color),
            fields_tile: Polygon::new(self.fields_tile_color),
            forest_tile: Polygon::new(self.forest_tile_color),
            coordinate_text: Text {
                color: self.coordinate_text_color,
                font_size: self.coordinate_text_font_size,
                round: self.coordinate_text_round,
            },
            roll_token_text: Text {
                color: self.roll_token_text_color,
                font_size: self.roll_token_text_font_size,
                round: self.roll_token_text_round,
            },
            hexagon_nominal_size: self.hexagon_nominal_size,
            hexagon_actual_size: self.hexagon_actual_size,
        }
    }
}

impl BoardViewSettings {
    pub fn new(
        upper_left_anchor: Vec2d,
        scale_width: Scalar,
        scale_height: Scalar,
        origin: Vec2d,
    ) -> BoardViewSettings {
        let default = BoardViewSettings::default();

        BoardViewSettings {
            upper_left_anchor,
            origin,
            scale_width,
            scale_height,
            ..default
        }
    }
}

impl Default for BoardViewSettings {
    fn default() -> BoardViewSettings {
        BoardViewSettings {
            upper_left_anchor: [0.0, 0.0],
            origin: [0.0, 0.0],
            scale_width: 1.0,
            scale_height: 1.0,
            building_tile_color: BUILDING_GREY,
            desert_tile_color: DESERT_YELLOW,
            mountain_tile_color: MOUNTAIN_BLUE_GREY,
            hill_tile_color: HILL_CLAY_ORANGE,
            pasture_tile_color: PASTURE_GREEN,
            fields_tile_color: FIELDS_WHEAT_YELLOW,
            forest_tile_color: FOREST_GREEN,
            coordinate_text_font_size: 18,
            coordinate_text_color: BLACK,
            coordinate_text_round: false,
            roll_token_text_font_size: 18,
            roll_token_text_color: BLACK,
            roll_token_text_round: false,
            hexagon_nominal_size: 50.0,
            hexagon_actual_size: 50.0 * 0.85,
        }
    }
}

impl BoardView {
    fn get_polygon_for_tile_type(&self, tile_type: &InternalTileType) -> &Polygon {
        match tile_type {
            &InternalTileType::BuildingTile(None) => &self.building_tile,
            &InternalTileType::BuildingTile(Some(_)) => &self.building_tile,
            &InternalTileType::ResourceTile(ResourceTileType::Desert) => &self.desert_tile,
            &InternalTileType::ResourceTile(ResourceTileType::Mountains) => &self.mountain_tile,
            &InternalTileType::ResourceTile(ResourceTileType::Hills) => &self.hill_tile,
            &InternalTileType::ResourceTile(ResourceTileType::Pasture) => &self.pasture_tile,
            &InternalTileType::ResourceTile(ResourceTileType::Fields) => &self.fields_tile,
            &InternalTileType::ResourceTile(ResourceTileType::Forest) => &self.forest_tile,
        }
    }
}

impl Renderer for BoardController {
    type Model = Board;
    type View = BoardView;

    fn render<C, G>(
        &self,
        board: &Board,
        board_view: &BoardView,
        context: &Context,
        glyphs: &mut C,
        g: &mut G,
    ) where
        C: CharacterCache,
        C::Error: fmt::Debug,
        G: Graphics<Texture = C::Texture>,
    {
        let centered_context = context
            .trans(
                board_view.upper_left_anchor[0],
                board_view.upper_left_anchor[1],
            )
            .scale(board_view.scale_width, board_view.scale_height)
            .trans(board_view.origin[0], board_view.origin[1]);

        for &coord in board.tiles.keys() {
            let tile_type = board.tiles.get(&coord).expect(
                "No tile found for coordinate!",
            );
            let possible_roll_token = board.roll_tokens.get(&coord);

            let polygon = board_view.get_polygon_for_tile_type(&tile_type);
            let center = convert_cube_coord_to_cartesian(coord, board_view.hexagon_nominal_size);
            let vertices = hexagon_vertices(center, board_view.hexagon_actual_size);
            polygon.draw(
                &vertices,
                &centered_context.draw_state,
                centered_context.transform,
                g,
            );

            if self.render_coordinate_text {
                render_text(
                    coord,
                    center,
                    board_view.coordinate_text,
                    &centered_context,
                    glyphs,
                    g,
                );
            }

            if self.render_roll_tokens {
                if let Some(roll_token) = possible_roll_token {
                    render_text(
                        roll_token,
                        center,
                        board_view.roll_token_text,
                        &centered_context,
                        glyphs,
                        g,
                    );
                }
            }
        }
    }
}

fn render_text<C, G, T>(
    object: T,
    position: Vec2d,
    text_object: Text,
    context: &Context,
    glyphs: &mut C,
    g: &mut G,
) -> Result<(), C::Error>
where
    T: fmt::Display,
    C: CharacterCache,
    C::Error: fmt::Debug,
    G: Graphics<Texture = C::Texture>,
{
    let text_content = format!("{}", object);
    let text_width = glyphs
        .width(text_object.font_size, text_content.as_str())
        .unwrap();
    let text_transform = context.transform.trans(
        position[0] - (text_width / 2.0),
        position[1] + (text_object.font_size as Scalar / 4.0),
    );

    text_object.draw(
        text_content.as_str(),
        glyphs,
        &context.draw_state,
        text_transform,
        g,
    )
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
