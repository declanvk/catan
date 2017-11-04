use piston_window::types::Color;

pub use piston_window::color::WHITE;
pub use piston_window::color::BLACK;
pub use piston_window::color::TRANSPARENT;

use catan_core::game::{PlayerColor, ResourceType};
use catan_core::board::{HarborType, ResourceTileType, BuildingTileContainer};

pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];

pub const BACKGROUND_SEA_BLUE: Color = [41.0 / 255.0, 182.0 / 255.0, 246.0 / 255.0, 1.0];

pub const BUILDING_GREY: Color = [238.0 / 255.0, 238.0 / 255.0, 238.0 / 255.0, 1.0];

pub const DESERT_YELLOW: Color = [252.0 / 255.0, 243.0 / 255.0, 207.0 / 255.0, 1.0];
pub const MOUNTAIN_BLUE_GREY: Color = [93.0 / 255.0, 109.0 / 255.0, 126.0 / 255.0, 1.0];
pub const HILL_CLAY_ORANGE: Color = [211.0 / 255.0, 84.0 / 255.0, 0.0, 1.0];
pub const PASTURE_GREEN: Color = [130.0 / 255.0, 224.0 / 255.0, 170.0 / 255.0, 1.0];
pub const FIELDS_WHEAT_YELLOW: Color = [244.0 / 255.0, 208.0 / 255.0, 63.0 / 255.0, 1.0];
pub const FOREST_GREEN: Color = [25.0 / 255.0, 111.0 / 255.0, 61.0 / 255.0, 1.0];

pub const PLAYER_ORANGE: Color = [255.0 / 255.0, 111.0 / 255.0, 0.0 / 255.0, 1.0];
pub const PLAYER_WHITE: Color = [250.0 / 255.0, 250.0 / 255.0, 250.0 / 255.0, 1.0];
pub const PLAYER_RED: Color = [183.0 / 255.0, 28.0 / 255.0, 28.0 / 255.0, 1.0];
pub const PLAYER_BLUE: Color = [26.0 / 255.0, 35.0 / 255.0, 126.0 / 255.0, 1.0];

pub const PERCEPTUAL_RAINBOW: [Color; 16] = [
    [135.0 / 255.0, 59.0 / 255.0, 97.0 / 255.0, 1.0],
    [143.0 / 255.0, 64.0 / 255.0, 127.0 / 255.0, 1.0],
    [143.0 / 255.0, 72.0 / 255.0, 157.0 / 255.0, 1.0],
    [135.0 / 255.0, 85.0 / 255.0, 185.0 / 255.0, 1.0],
    [121.0 / 255.0, 102.0 / 255.0, 207.0 / 255.0, 1.0],
    [103.0 / 255.0, 123.0 / 255.0, 220.0 / 255.0, 1.0],
    [84.0 / 255.0, 146.0 / 255.0, 223.0 / 255.0, 1.0],
    [69.0 / 255.0, 170.0 / 255.0, 215.0 / 255.0, 1.0],
    [59.0 / 255.0, 192.0 / 255.0, 197.0 / 255.0, 1.0],
    [60.0 / 255.0, 210.0 / 255.0, 172.0 / 255.0, 1.0],
    [71.0 / 255.0, 223.0 / 255.0, 145.0 / 255.0, 1.0],
    [93.0 / 255.0, 229.0 / 255.0, 120.0 / 255.0, 1.0],
    [124.0 / 255.0, 231.0 / 255.0, 103.0 / 255.0, 1.0],
    [161.0 / 255.0, 227.0 / 255.0, 95.0 / 255.0, 1.0],
    [198.0 / 255.0, 220.0 / 255.0, 100.0 / 255.0, 1.0],
    [233.0 / 255.0, 213.0 / 255.0, 117.0 / 255.0, 1.0],
];

pub fn player_to_color(player: &PlayerColor) -> Color {
    match *player {
        PlayerColor::Red => PLAYER_RED,
        PlayerColor::White => PLAYER_WHITE,
        PlayerColor::Orange => PLAYER_ORANGE,
        PlayerColor::Blue => PLAYER_BLUE,
    }
}

pub fn resource_to_color(resource_type: &ResourceType) -> Color {
    match *resource_type {
        ResourceType::Brick => HILL_CLAY_ORANGE,
        ResourceType::Grain => FIELDS_WHEAT_YELLOW,
        ResourceType::Lumber => FOREST_GREEN,
        ResourceType::Ore => MOUNTAIN_BLUE_GREY,
        ResourceType::Wool => PASTURE_GREEN,
    }
}

pub fn resource_tile_to_color(resource_tile: &ResourceTileType) -> Color {
    match resource_tile.into_resource_type() {
        None => DESERT_YELLOW,
        Some(resource_type) => resource_to_color(&resource_type),
    }
}

pub fn harbor_type_to_color(harbor_type: &HarborType) -> Color {
    match harbor_type.into_resource_type() {
        None => BACKGROUND_SEA_BLUE,
        Some(resource_type) => resource_to_color(&resource_type),
    }
}

pub fn distance_to_color(distance: i32) -> Color {
    let corrected_index = (((distance) + 16) % 16) as usize;

    PERCEPTUAL_RAINBOW[corrected_index]
}

pub fn lerp(a: Color, b: Color) -> Color {
    [
        (a[0] + b[0]) / 2.0,
        (a[1] + b[1]) / 2.0,
        (a[2] + b[2]) / 2.0,
        (a[3] + b[3]) / 2.0,
    ]
}
