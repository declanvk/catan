use std::collections::HashMap;
use std::fmt;

const BOARD_RESOURCE_TILE_RADIUS: u32 = 3;

#[derive(PartialEq, Eq, Hash)]
pub struct InternalCoord {
    x: i32,
    y: i32,
    z: i32,
}

const TILE_COORD_DIR: [(i32, i32, i32); 6] = [
    (1, -1, 0),
    (1, 0, -1),
    (0, 1, -1),
    (-1, 1, 0),
    (-1, 0, 1),
    (0, -1, 1),
];

const TILE_COORD_DIAGONAL_DIR: [(i32, i32, i32); 6] = [
    (1, 1, -2),
    (2, -1, -1),
    (1, -2, 1),
    (-1, -1, 2),
    (-2, 1, 1),
    (-1, 2, -1),
];

impl InternalCoord {
    pub fn new(x: i32, y: i32, z: i32) -> InternalCoord {
        assert_eq!(x + y + z, 0, "Coordinates do not sum to zero!");

        InternalCoord { x, y, z }
    }

    pub fn adjacent(&self, other: &InternalCoord) -> bool {
        self.distance(other) == 2.0
    }

    pub fn distance(&self, other: &InternalCoord) -> f32 {
        ((self.x as f32 - other.x as f32).abs() + (self.y as f32 - other.y as f32).abs() +
            (self.z as f32 - other.z as f32).abs())
    }

    pub fn neighbors(&self) -> Vec<InternalCoord> {
        let mut neighbors: Vec<InternalCoord> = Vec::new();

        for direction in TILE_COORD_DIR.iter() {
            let (dx, dy, dz) = *direction;

            neighbors.push(InternalCoord::new(self.x + dx, self.y + dy, self.z + dz))
        }

        neighbors
    }

    pub fn diagonal_neighbors(&self) -> Vec<InternalCoord> {
        let mut diagonals: Vec<InternalCoord> = Vec::new();

        for direction in TILE_COORD_DIAGONAL_DIR.iter() {
            let (dx, dy, dz) = *direction;

            diagonals.push(InternalCoord::new(self.x + dx, self.y + dy, self.z + dz))
        }

        diagonals
    }
}

impl fmt::Debug for InternalCoord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Tile: ({}, {}, {})>", self.x, self.y, self.z)
    }
}


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Ore,
    Brick,
    Grain,
    Wool,
    Lumber,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TileType {
    Mountains,
    Hills,
    Pasture,
    Fields,
    Forest,
    Desert,
}

impl TileType {
    fn resource(self) -> Option<ResourceType> {
        match self {
            TileType::Mountains => Some(ResourceType::Ore),
            TileType::Hills => Some(ResourceType::Brick),
            TileType::Pasture => Some(ResourceType::Wool),
            TileType::Fields => Some(ResourceType::Grain),
            TileType::Forest => Some(ResourceType::Lumber),
            TileType::Desert => None,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    tiles: HashMap<InternalCoord, TileType>,
    roll_tokens: HashMap<InternalCoord, u32>,
    robber_coord: InternalCoord,
}

impl Board {
    fn balanced_board() -> Board {
        let mut tiles = HashMap::new();
        let mut roll_tokens = HashMap::new();



        Board {
            tiles,
            roll_tokens,
            robber_coord: InternalCoord::new(0, 0, 0),
        }
    }

    fn random_board() -> Board {
        let mut tiles = HashMap::new();
        let mut roll_tokens = HashMap::new();

        Board {
            tiles,
            roll_tokens,
            robber_coord: InternalCoord::new(0, 0, 0),
        }
    }
}
