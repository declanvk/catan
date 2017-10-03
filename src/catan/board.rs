use std::collections::{HashMap, HashSet};
use std::fmt;
use catan::common::Resource;
use rand;
use rand::Rng;

const BOARD_RESOURCE_TILE_RADIUS: u32 = 3;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct InternalCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
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
    (2, -1, -1),
    (1, 1, -2),
    (-1, 2, -1),
    (-2, 1, 1),
    (-1, -1, 2),
    (1, -2, 1),
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum InternalTileType {
    BuildingTile,
    ResourceTile(ResourceTileType),
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ResourceType {
    Ore,
    Brick,
    Grain,
    Wool,
    Lumber,
}

impl Resource for ResourceType {
    fn count(self) -> usize {
        match self {
            ResourceType::Ore => 19,
            ResourceType::Brick => 19,
            ResourceType::Grain => 19,
            ResourceType::Wool => 19,
            ResourceType::Lumber => 19,
        }
    }

    fn all_variants() -> HashSet<ResourceType> {
        let mut variants: HashSet<ResourceType> = HashSet::new();

        variants.insert(ResourceType::Ore);
        variants.insert(ResourceType::Brick);
        variants.insert(ResourceType::Grain);
        variants.insert(ResourceType::Wool);
        variants.insert(ResourceType::Lumber);

        variants
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ResourceTileType {
    Mountains,
    Hills,
    Pasture,
    Fields,
    Forest,
    Desert,
}

impl ResourceTileType {
    pub fn resource(self) -> Option<ResourceType> {
        match self {
            ResourceTileType::Mountains => Some(ResourceType::Ore),
            ResourceTileType::Hills => Some(ResourceType::Brick),
            ResourceTileType::Pasture => Some(ResourceType::Wool),
            ResourceTileType::Fields => Some(ResourceType::Grain),
            ResourceTileType::Forest => Some(ResourceType::Lumber),
            ResourceTileType::Desert => None,
        }
    }
}

impl Resource for ResourceTileType {
    fn count(self) -> usize {
        match self {
            ResourceTileType::Mountains => 3,
            ResourceTileType::Hills => 3,
            ResourceTileType::Pasture => 4,
            ResourceTileType::Fields => 4,
            ResourceTileType::Forest => 4,
            ResourceTileType::Desert => 2,
        }
    }

    fn all_variants() -> HashSet<ResourceTileType> {
        let mut variants: HashSet<ResourceTileType> = HashSet::new();

        variants.insert(ResourceTileType::Mountains);
        variants.insert(ResourceTileType::Hills);
        variants.insert(ResourceTileType::Pasture);
        variants.insert(ResourceTileType::Fields);
        variants.insert(ResourceTileType::Forest);
        variants.insert(ResourceTileType::Desert);

        variants
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RollToken {
    Two,
    Three,
    Four,
    Five,
    Six,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

impl RollToken {
    pub fn new(value: u32) -> RollToken {
        match value {
            2 => RollToken::Two,
            3 => RollToken::Three,
            4 => RollToken::Four,
            5 => RollToken::Five,
            6 => RollToken::Six,
            8 => RollToken::Eight,
            9 => RollToken::Nine,
            10 => RollToken::Ten,
            11 => RollToken::Eleven,
            12 => RollToken::Twelve,
            _ => {
                panic!("Invalid RollToken value!");
            }
        }
    }

    pub fn value(self) -> u32 {
        match self {
            RollToken::Two => 2,
            RollToken::Three => 3,
            RollToken::Four => 4,
            RollToken::Five => 5,
            RollToken::Six => 6,
            RollToken::Eight => 8,
            RollToken::Nine => 9,
            RollToken::Ten => 10,
            RollToken::Eleven => 11,
            RollToken::Twelve => 12,
        }
    }

    pub fn high_prob(&self) -> bool {
        match self {
            &RollToken::Six |
            &RollToken::Eight => true,
            _ => false,
        }
    }
}

impl PartialEq<u32> for RollToken {
    fn eq(&self, other: &u32) -> bool {
        match self {
            &RollToken::Two => *other == 2,
            &RollToken::Three => *other == 3,
            &RollToken::Four => *other == 4,
            &RollToken::Five => *other == 5,
            &RollToken::Six => *other == 6,
            &RollToken::Eight => *other == 8,
            &RollToken::Nine => *other == 9,
            &RollToken::Ten => *other == 10,
            &RollToken::Eleven => *other == 11,
            &RollToken::Twelve => *other == 12,
        }
    }
}

impl Resource for RollToken {
    fn count(self) -> usize {
        match self {
            RollToken::Two => 1,
            RollToken::Three => 2,
            RollToken::Four => 2,
            RollToken::Five => 2,
            RollToken::Six => 2,
            RollToken::Eight => 2,
            RollToken::Nine => 2,
            RollToken::Ten => 2,
            RollToken::Eleven => 2,
            RollToken::Twelve => 1,
        }
    }

    fn all_variants() -> HashSet<RollToken> {
        let mut variants: HashSet<RollToken> = HashSet::new();

        variants.insert(RollToken::Two);
        variants.insert(RollToken::Three);
        variants.insert(RollToken::Four);
        variants.insert(RollToken::Five);
        variants.insert(RollToken::Six);
        variants.insert(RollToken::Eight);
        variants.insert(RollToken::Nine);
        variants.insert(RollToken::Ten);
        variants.insert(RollToken::Eleven);
        variants.insert(RollToken::Twelve);

        variants
    }
}

const STANDARD_TILE_LOCATIONS: [InternalCoord; 17] = [
    InternalCoord { x: 0, y: 0, z: 0 },
    InternalCoord { x: 2, y: -1, z: -1 },
    InternalCoord { x: 1, y: 1, z: -2 },
    InternalCoord { x: -1, y: 2, z: -1 },
    InternalCoord { x: -2, y: 1, z: 1 },
    InternalCoord { x: -1, y: -1, z: 2 },
    InternalCoord { x: 1, y: -2, z: 1 },
    InternalCoord { x: 3, y: -3, z: 0 },
    InternalCoord { x: -3, y: 3, z: 0 },
    InternalCoord { x: 0, y: 3, z: -3 },
    InternalCoord { x: 0, y: -3, z: 3 },
    InternalCoord { x: -4, y: 2, z: 2 },
    InternalCoord { x: 4, y: -2, z: -2 },
    InternalCoord { x: 2, y: 2, z: -4 },
    InternalCoord { x: -2, y: -2, z: 4 },
    InternalCoord { x: -2, y: 4, z: -2 },
    InternalCoord { x: 2, y: -4, z: 2 },
];

#[derive(Debug)]
pub struct Board {
    pub tiles: HashMap<InternalCoord, InternalTileType>,
    pub roll_tokens: HashMap<InternalCoord, RollToken>,
    pub robber_coord: InternalCoord,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: HashMap::new(),
            roll_tokens: HashMap::new(),
            robber_coord: InternalCoord::new(0, 0, 0),
        }
    }

    pub fn random_start() -> Board {
        let mut board = Board::new();
        let mut resource_tile_types = ResourceTileType::full_shuffled_collection();

        let (mut high_prob_roll_tokens, mut low_prob_roll_tokens): (Vec<RollToken>,
                                                                    Vec<RollToken>) =
            RollToken::full_shuffled_collection()
                .into_iter()
                .partition(|&token| token.high_prob());

        for &location in &STANDARD_TILE_LOCATIONS {
            let tile_type = resource_tile_types.pop().expect("Not enough tile types!");
            board.tiles.insert(
                location,
                InternalTileType::ResourceTile(tile_type),
            );

            let adjacent_high_prob = location.diagonal_neighbors().into_iter().any(|coord| {
                let adjacent_roll_token = board.roll_tokens.get(&coord);

                match adjacent_roll_token {
                    Some(&roll_token) => roll_token.high_prob(),
                    None => false,
                }
            });

            if tile_type != ResourceTileType::Desert {
                let high_low_tokens_difference =
                    (low_prob_roll_tokens.len() - high_prob_roll_tokens.len()) as u32;

                let roll_token = if adjacent_high_prob {
                    low_prob_roll_tokens.pop().expect(
                        "Not enough low prob tokens!",
                    )
                } else {
                    if high_prob_roll_tokens.len() == 0 {
                        low_prob_roll_tokens.pop().expect("Not enough tokens!")
                    } else if low_prob_roll_tokens.len() == 0 {
                        high_prob_roll_tokens.pop().expect("Not enough tokens!")
                    } else if rand::thread_rng().gen_weighted_bool(high_low_tokens_difference) {
                        low_prob_roll_tokens.pop().expect(
                            "Not enough low prob tokens!",
                        )
                    } else {
                        high_prob_roll_tokens.pop().expect(
                            "Not enough high prob tokens",
                        )
                    }
                };

                board.roll_tokens.insert(location, roll_token);
            } else {
                board.robber_coord = location;
            }

            for neighbor in location.neighbors() {
                if !board.tiles.contains_key(&neighbor) {
                    board.tiles.insert(neighbor, InternalTileType::BuildingTile);
                }
            }
        }

        board
    }
}
