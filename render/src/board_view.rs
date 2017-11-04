use piston_window::*;
use piston_window::character::CharacterCache;
use piston_window::math::{Scalar, Vec2d};
use piston_window::types::{Color, FontSize};
use piston_window::context::Context;


use catan_core::board::{InternalCoord, Board, InternalTileType, ResourceTileType,
                        BuildingTileContainer, HarborType, TILE_COORD_DIR};
use catan_core::game::PlayerColor;
use catan_core::common::GameResource;
use colors::*;
use common::{Renderer, Controller, Builder};
use std::collections::HashMap;

use std::fmt;
use std::default::Default;

use log;

pub struct BoardController {
    render_coordinate_text: bool,
    render_roll_tokens: bool,
    render_view_borders: bool,
    render_distance_from_center: bool,
}

impl BoardController {
    pub fn new(
        render_coordinate_text: bool,
        render_roll_tokens: bool,
        render_view_borders: bool,
        render_distance_from_center: bool,
    ) -> BoardController {
        BoardController {
            render_coordinate_text,
            render_roll_tokens,
            render_view_borders,
            render_distance_from_center,
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
        } else if button == Button::Keyboard(Key::NumPad3) || button == Button::Keyboard(Key::D3) {
            self.render_view_borders = !self.render_view_borders;
        } else if button == Button::Keyboard(Key::NumPad4) || button == Button::Keyboard(Key::D4) {
            self.render_distance_from_center = !self.render_distance_from_center;
        });

    }
}

pub struct BoardView {
    upper_left_anchor: Vec2d,
    width: Scalar,
    height: Scalar,
    polygon_container: BoardPolygonContainer,
    coordinate_text: Text,
    roll_token_text: Text,
    hexagon_nominal_size: Scalar,
    hexagon_actual_size: Scalar,
}

pub struct BoardViewSettings {
    upper_left_anchor: Vec2d,
    width: Scalar,
    height: Scalar,
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
            width: self.width,
            height: self.height,
            polygon_container: BoardPolygonContainer::default(),
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
    pub fn new(upper_left_anchor: Vec2d, width: Scalar, height: Scalar) -> BoardViewSettings {
        let default = BoardViewSettings::default();

        BoardViewSettings {
            upper_left_anchor,
            width,
            height,
            ..default
        }
    }
}

impl Default for BoardViewSettings {
    fn default() -> BoardViewSettings {
        BoardViewSettings {
            upper_left_anchor: [0.0, 0.0],
            width: 400.0,
            height: 400.0,
            coordinate_text_font_size: 18,
            coordinate_text_color: BLACK,
            coordinate_text_round: false,
            roll_token_text_font_size: 18,
            roll_token_text_color: BLACK,
            roll_token_text_round: false,
            hexagon_nominal_size: 40.0,
            hexagon_actual_size: 40.0 * 0.85,
        }
    }
}

struct BoardPolygonContainer {
    tile_polygons: HashMap<InternalTileType, Polygon>,
    harbor_polygons: HashMap<HarborType, Polygon>,
    distance_polygons: HashMap<i32, Polygon>,
}

impl BoardPolygonContainer {
    fn new() -> BoardPolygonContainer {
        BoardPolygonContainer {
            tile_polygons: HashMap::new(),
            harbor_polygons: HashMap::new(),
            distance_polygons: HashMap::new(),
        }
    }

    fn get_tile_polygon(&mut self, tile_type: &InternalTileType) -> &Polygon {
        if self.tile_polygons.contains_key(tile_type) {
            return self.tile_polygons.get(tile_type).unwrap();
        } else {
            let new_polygon = match *tile_type {
                InternalTileType::BuildingTile(building_tile) => {
                    if let Some(harbor_type) = building_tile.harbor_type {
                        let color = lerp(lerp(harbor_type_to_color(&harbor_type), BUILDING_GREY), BUILDING_GREY);
                        Polygon::new(color)
                    } else {
                        Polygon::new(BUILDING_GREY)
                    }
                }
                InternalTileType::ResourceTile(resource_tile_type) => {
                    let color = resource_tile_to_color(&resource_tile_type);
                    Polygon::new(color)
                }
            };

            self.tile_polygons.insert(*tile_type, new_polygon);

            self.tile_polygons.get(tile_type).unwrap()
        }
    }

    fn get_harbor_polygon(&mut self, harbor_type: &HarborType) -> &Polygon {
        if self.harbor_polygons.contains_key(harbor_type) {
            return self.harbor_polygons.get(harbor_type).unwrap();
        } else {
            let new_polygon = Polygon::new(harbor_type_to_color(harbor_type));

            self.harbor_polygons.insert(*harbor_type, new_polygon);

            self.harbor_polygons.get(harbor_type).unwrap()
        }
    }

    fn get_distance_polygon(&mut self, distance: f32) -> &Polygon {
        let rounded_distance = distance.round() as i32;
        if self.distance_polygons.contains_key(&rounded_distance) {
            return self.distance_polygons.get(&rounded_distance).unwrap();
        } else {
            let new_polygon = Polygon::new(distance_to_color(rounded_distance));

            self.distance_polygons.insert(rounded_distance, new_polygon);

            self.distance_polygons.get(&rounded_distance).unwrap()
        }
    }
}

impl Default for BoardPolygonContainer {
    fn default() -> BoardPolygonContainer {
        let mut container = BoardPolygonContainer::new();

        for resource_tile_type in ResourceTileType::all_variants().iter() {
            let color = resource_tile_to_color(resource_tile_type);
            container.tile_polygons.insert(
                InternalTileType::ResourceTile(*resource_tile_type),
                Polygon::new(color),
            );
        }

        container
    }
}

const BOARD_ROTATION_DEGREES: f64 = -30.0;

impl Renderer for BoardController {
    type Model = Board;
    type View = BoardView;

    fn render<C, G>(
        &self,
        board: &Board,
        board_view: &mut BoardView,
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
            .trans(board_view.width / 2.0, board_view.height / 2.0)
            .rot_deg(BOARD_ROTATION_DEGREES);

        for &coord in board.tiles.keys() {
            let (tile_type, possible_roll_token) = board.get_location(coord);

            let polygon = if self.render_distance_from_center {
                let distance = coord.distance(&InternalCoord::new(0, 0, 0));
                board_view.polygon_container.get_distance_polygon(distance)
            } else {
                board_view.polygon_container.get_tile_polygon(&tile_type)
            };

            let center = convert_cube_coord_to_cartesian(coord, board_view.hexagon_nominal_size);
            let hexagon_context = centered_context.trans(center[0], center[1]).zoom(
                board_view.hexagon_actual_size,
            );
            let vertices = hexagon_vertices(HEX_ROTATION_DEGREES);
            polygon.draw(
                &vertices,
                &hexagon_context.draw_state,
                hexagon_context.transform,
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

            if self.render_view_borders {
                Rectangle::new_border(RED, 1.0).draw(
                    [
                        board_view.upper_left_anchor[0],
                        board_view.upper_left_anchor[1],
                        board_view.width,
                        board_view.height,
                    ],
                    &context.draw_state,
                    context.transform,
                    g,
                )
            }
        }

        for (&harbor_coord, &(harbor_type, axis)) in board.harbors.iter() {
            let polygon = board_view.polygon_container.get_harbor_polygon(
                &harbor_type,
            );
            let center =
                convert_cube_coord_to_cartesian(harbor_coord, board_view.hexagon_nominal_size);
            let harbor_context = centered_context.trans(center[0], center[1]).zoom(
                board_view.hexagon_actual_size,
            );
            let vertices = rhombus_vertices(axis, HEX_ROTATION_DEGREES);

            polygon.draw(
                &vertices,
                &harbor_context.draw_state,
                harbor_context.transform,
                g,
            );
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
    let text_transform = context
        .transform
        .trans(position[0], position[1])
        .rot_deg(-BOARD_ROTATION_DEGREES)
        .trans(-(text_width / 2.0), (text_object.font_size as Scalar / 4.0));

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

const HEX_ROTATION_DEGREES: u32 = 30;

fn hexagon_vertices(rotation: u32) -> Box<[Vec2d]> {
    (0..7)
        .map(|ind| {
            [
                ((60 * ind + rotation) as Scalar).to_radians().cos(),
                ((60 * ind + rotation) as Scalar).to_radians().sin(),
            ]
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn rhombus_vertices(axis: u32, rotation: u32) -> Box<[Vec2d]> {
    let mut vertices = vec![[0.0, 0.0]];
    vertices.extend_from_slice(
        (0..3)
            .map(|ind| {
                [
                    ((60 * (axis + ind) + rotation) as Scalar)
                        .to_radians()
                        .cos(),
                    ((60 * (axis + ind) + rotation) as Scalar)
                        .to_radians()
                        .sin(),
                ]
            })
            .collect::<Vec<_>>()
            .as_slice(),
    );

    vertices.into_boxed_slice()
}
