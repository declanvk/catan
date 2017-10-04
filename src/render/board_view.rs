use piston_window::*;
use piston_window::character::CharacterCache;
use piston_window::math::{Scalar, Vec2d};

use catan::board::{InternalCoord, Board, InternalTileType, ResourceTileType};
use render::colors::*;
use render::common::RenderView;

use std::fmt;
use std::default::Default;

pub struct BoardView<'a> {
    board: &'a Board,
}

impl<'a> BoardView<'a> {
    pub fn new(board: &'a Board) -> BoardView<'a> {
        BoardView { board }
    }
}

pub struct BoardViewSettings {
    building_tile: Polygon,
    desert_tile: Polygon,
    mountain_tile: Polygon,
    hill_tile: Polygon,
    pasture_tile: Polygon,
    fields_tile: Polygon,
    forest_tile: Polygon,
    text: Text,
    hexagon_nominal_size: Scalar,
    hexagon_actual_size: Scalar,
    render_text: bool,
}

impl Default for BoardViewSettings {
    fn default() -> BoardViewSettings {
        BoardViewSettings {
            building_tile: Polygon::new(BUILDING_GREY),
            desert_tile: Polygon::new(DESERT_YELLOW),
            mountain_tile: Polygon::new(MOUNTAIN_BLUE_GREY),
            hill_tile: Polygon::new(HILL_CLAY_ORANGE),
            pasture_tile: Polygon::new(PASTURE_GREEN),
            fields_tile: Polygon::new(FIELDS_WHEAT_YELLOW),
            forest_tile: Polygon::new(FOREST_GREEN),
            text: Text {
                font_size: 18,
                color: BLACK,
                round: false,
            },
            hexagon_nominal_size: 46.0,
            hexagon_actual_size: 46.0 * 0.85,
            render_text: false,
        }
    }
}

impl BoardViewSettings {
    pub fn set_render_text(&mut self, render_text: bool) {
        self.render_text = render_text
    }

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

impl<'a> RenderView for BoardView<'a> {
    type Settings = BoardViewSettings;

    fn render<C, G>(
        &self,
        settings: &BoardViewSettings,
        context: &Context,
        glyphs: &mut C,
        g: &mut G,
    ) where
        C: CharacterCache,
        C::Error: fmt::Debug,
        G: Graphics<Texture = C::Texture>,
    {
        for entry in self.board.tiles.iter() {
            entry.render(settings, context, glyphs, g);
        }
    }
}

impl<'a> RenderView for (&'a InternalCoord, &'a InternalTileType) {
    type Settings = BoardViewSettings;

    fn render<C, G>(
        &self,
        settings: &BoardViewSettings,
        context: &Context,
        glyphs: &mut C,
        g: &mut G,
    ) where
        C: CharacterCache,
        C::Error: fmt::Debug,
        G: Graphics<Texture = C::Texture>,
    {
        let &(&coord, tile_type) = self;
        let polygon = settings.get_polygon_for_tile_type(tile_type);
        let center = convert_cube_coord_to_cartesian(coord, settings.hexagon_nominal_size);
        let vertices = hexagon_vertices(center, settings.hexagon_actual_size);
        polygon.draw(&vertices, &context.draw_state, context.transform, g);

        let text_content = format!("{}", coord);
        let text_width = glyphs
            .width(settings.text.font_size, text_content.as_str())
            .unwrap();
        let text_transform = context.transform.trans(
            center[0] - (text_width / 2.0),
            center[1] + (settings.text.font_size as Scalar / 4.0),
        );

        if settings.render_text {
            settings.text.draw(
                text_content.as_str(),
                glyphs,
                &context.draw_state,
                text_transform,
                g,
            );
        }
    }
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
