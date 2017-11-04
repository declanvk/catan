use std::collections::{HashMap, HashSet};
use std::fmt;
use common::GameResource;
use super::game::{ResourceType, PlayerColor};
use rand;
use rand::Rng;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct InternalCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub const TILE_COORD_DIR: [(i32, i32, i32); 6] = [
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

    // pub fn adjacent_facing_tiles(
    //     a: &InternalCoord,
    //     b: &InternalCoord,
    // ) -> (InternalCoord, InternalCoord) {
    //     assert!(a.adjacent(b), "a and b must be adjacent!");

    //     let a_neighbors: HashSet<_> = a.neighbors().into_iter().collect();
    //     let b_neighbors: HashSet<_> = b.neighbors().into_iter().collect();

    //     let common_neighbors: Vec<_> = a_neighbors.intersection(&b_neighbors).take(2).collect();
    //     assert!(
    //         common_neighbors.len() == 2,
    //         "Incorrect number of common neighbors!"
    //     );

    //     (
    //         *common_neighbors
    //             .get(0)
    //             .expect("Failed to get first common neighbor")
    //             .clone(),
    //         *common_neighbors
    //             .get(1)
    //             .expect("Failed to get second common neighbor")
    //             .clone(),
    //     )
    // }
}

impl fmt::Debug for InternalCoord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Tile: ({}, {}, {})>", self.x, self.y, self.z)
    }
}

impl fmt::Display for InternalCoord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum InternalTileType {
    BuildingTile(BuildingTileContainer),
    ResourceTile(ResourceTileType),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BuildingTileContainer {
    pub building: Option<(PlayerColor, BuildingType)>,
    pub harbor_type: Option<HarborType>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum HarborType {
    All,
    Ore,
    Brick,
    Wool,
    Grain,
    Lumber,
}

impl HarborType {
    pub fn into_resource_type(&self) -> Option<ResourceType> {
        match *self {
            HarborType::All => None,
            HarborType::Brick => Some(ResourceType::Brick),
            HarborType::Grain => Some(ResourceType::Grain),
            HarborType::Lumber => Some(ResourceType::Lumber),
            HarborType::Ore => Some(ResourceType::Ore),
            HarborType::Wool => Some(ResourceType::Wool),
        }
    }
}

impl GameResource for HarborType {
    fn count(self) -> usize {
        match self {
            HarborType::All => 4,
            _ => 1,
        }
    }

    fn all_variants() -> HashSet<HarborType> {
        let mut variants: HashSet<HarborType> = HashSet::new();

        variants.insert(HarborType::All);
        variants.insert(HarborType::Ore);
        variants.insert(HarborType::Brick);
        variants.insert(HarborType::Wool);
        variants.insert(HarborType::Grain);
        variants.insert(HarborType::Lumber);

        variants
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BuildingType {
    Settlement,
    City,
    Road,
}

impl GameResource for BuildingType {
    fn count(self) -> usize {
        match self {
            BuildingType::Settlement => 5,
            BuildingType::City => 4,
            BuildingType::Road => 15,
        }
    }

    fn all_variants() -> HashSet<BuildingType> {
        let mut variants: HashSet<BuildingType> = HashSet::new();

        variants.insert(BuildingType::Settlement);
        variants.insert(BuildingType::City);
        variants.insert(BuildingType::Road);

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
    pub fn into_resource_type(self) -> Option<ResourceType> {
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

impl GameResource for ResourceTileType {
    fn count(self) -> usize {
        match self {
            ResourceTileType::Mountains => 3,
            ResourceTileType::Hills => 3,
            ResourceTileType::Pasture => 4,
            ResourceTileType::Fields => 4,
            ResourceTileType::Forest => 4,
            ResourceTileType::Desert => 1,
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
    Seven,
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
            7 => RollToken::Seven,
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
            RollToken::Seven => 7,
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

impl fmt::Display for RollToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
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
            &RollToken::Seven => *other == 7,
            &RollToken::Eight => *other == 8,
            &RollToken::Nine => *other == 9,
            &RollToken::Ten => *other == 10,
            &RollToken::Eleven => *other == 11,
            &RollToken::Twelve => *other == 12,
        }
    }
}

impl GameResource for RollToken {
    fn count(self) -> usize {
        match self {
            RollToken::Two => 1,
            RollToken::Three => 2,
            RollToken::Four => 2,
            RollToken::Five => 2,
            RollToken::Six => 2,
            RollToken::Seven => 0,
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
        variants.insert(RollToken::Seven);
        variants.insert(RollToken::Eight);
        variants.insert(RollToken::Nine);
        variants.insert(RollToken::Ten);
        variants.insert(RollToken::Eleven);
        variants.insert(RollToken::Twelve);

        variants
    }
}

const STANDARD_TILE_LOCATIONS: [InternalCoord; 19] = [
    InternalCoord { x: 0, y: 0, z: 0 }, // Center
    InternalCoord { x: 2, y: -1, z: -1 }, // Start first ring
    InternalCoord { x: 1, y: 1, z: -2 },
    InternalCoord { x: -1, y: 2, z: -1 },
    InternalCoord { x: -2, y: 1, z: 1 },
    InternalCoord { x: -1, y: -1, z: 2 },
    InternalCoord { x: 1, y: -2, z: 1 }, // End first ring
    InternalCoord { x: 3, y: -3, z: 0 }, // Start second ring
    InternalCoord { x: 4, y: -2, z: -2 },
    InternalCoord { x: 3, y: 0, z: -3 },
    InternalCoord { x: 2, y: 2, z: -4 },
    InternalCoord { x: 0, y: 3, z: -3 },
    InternalCoord { x: -2, y: 4, z: -2 },
    InternalCoord { x: -3, y: 3, z: 0 },
    InternalCoord { x: -4, y: 2, z: 2 },
    InternalCoord { x: -3, y: 0, z: 3 },
    InternalCoord { x: -2, y: -2, z: 4 },
    InternalCoord { x: 0, y: -3, z: 3 },
    InternalCoord { x: 2, y: -4, z: 2 },
];

const BALANCED_TILE_VALUES: [(ResourceTileType, Option<RollToken>); 19] =
    [
        (ResourceTileType::Desert, None),
        (ResourceTileType::Mountains, Some(RollToken::Twelve)),
        (ResourceTileType::Fields, Some(RollToken::Eleven)),
        (ResourceTileType::Hills, Some(RollToken::Three)),
        (ResourceTileType::Forest, Some(RollToken::Eight)),
        (ResourceTileType::Pasture, Some(RollToken::Nine)),
        (ResourceTileType::Forest, Some(RollToken::Two)),
        (ResourceTileType::Forest, Some(RollToken::Four)),
        (ResourceTileType::Hills, Some(RollToken::Six)),
        (ResourceTileType::Pasture, Some(RollToken::Ten)),
        (ResourceTileType::Fields, Some(RollToken::Nine)),
        (ResourceTileType::Pasture, Some(RollToken::Six)),
        (ResourceTileType::Forest, Some(RollToken::Five)),
        (ResourceTileType::Hills, Some(RollToken::Four)),
        (ResourceTileType::Pasture, Some(RollToken::Ten)),
        (ResourceTileType::Fields, Some(RollToken::Three)),
        (ResourceTileType::Mountains, Some(RollToken::Five)),
        (ResourceTileType::Fields, Some(RollToken::Eleven)),
        (ResourceTileType::Mountains, Some(RollToken::Eight)),
    ];

const BALANCED_HARBOR_LOCATIONS: [(InternalCoord, (HarborType, u32)); 9] =
    [
        (InternalCoord { x: 6, y: -3, z: -3 }, (HarborType::All, 1)), //
        (InternalCoord { x: 4, y: 1, z: -5 }, (HarborType::Brick, 2)), //
        (InternalCoord { x: 1, y: 4, z: -5 }, (HarborType::Lumber, 2)), //
        (InternalCoord { x: -3, y: 6, z: -3 }, (HarborType::All, 3)), //
        (InternalCoord { x: -5, y: 4, z: 1 }, (HarborType::Grain, 4)),
        (InternalCoord { x: -5, y: 1, z: 4 }, (HarborType::Ore, 4)),
        (InternalCoord { x: -3, y: -3, z: 6 }, (HarborType::All, 5)),
        (InternalCoord { x: 1, y: -5, z: 4 }, (HarborType::Wool, 0)), //
        (InternalCoord { x: 4, y: -5, z: 1 }, (HarborType::All, 0)),
    ];

const BALANCED_HARBOR_BUILDING_LOCATIONS: [(InternalCoord, InternalCoord); 9] =
    [
        (
            InternalCoord { x: 5, y: -3, z: -2 },
            InternalCoord { x: 5, y: -2, z: -3 },
        ),
        (
            InternalCoord { x: 4, y: 0, z: -4 },
            InternalCoord { x: 3, y: 1, z: -4 },
        ),
        (
            InternalCoord { x: 1, y: 3, z: -4 },
            InternalCoord { x: 0, y: 4, z: -4 },
        ),
        (
            InternalCoord { x: -2, y: 5, z: -3 },
            InternalCoord { x: -3, y: 5, z: -2 },
        ),
        (
            InternalCoord { x: -4, y: 4, z: 0 },
            InternalCoord { x: -4, y: 3, z: 1 },
        ),
        (
            InternalCoord { x: -4, y: 1, z: 3 },
            InternalCoord { x: -4, y: 0, z: 4 },
        ),
        (
            InternalCoord { x: -3, y: -2, z: 5 },
            InternalCoord { x: -2, y: -3, z: 5 },
        ),
        (
            InternalCoord { x: 0, y: -4, z: 4 },
            InternalCoord { x: 1, y: -4, z: 3 },
        ),
        (
            InternalCoord { x: 3, y: -4, z: 1 },
            InternalCoord { x: 4, y: -4, z: 0 },
        ),
    ];

#[derive(Debug)]
pub struct Board {
    pub tiles: HashMap<InternalCoord, InternalTileType>,
    pub roll_tokens: HashMap<InternalCoord, RollToken>,
    pub harbors: HashMap<InternalCoord, (HarborType, u32)>,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            tiles: HashMap::default(),
            roll_tokens: HashMap::default(),
            harbors: HashMap::default(),
        }
    }
}

impl Board {
    pub fn balanced_start() -> Board {
        let mut board = Board::default();

        let harbor_buildings: HashMap<InternalCoord, usize> = BALANCED_HARBOR_BUILDING_LOCATIONS
            .iter()
            .cloned()
            .enumerate()
            .flat_map(|(index, (coord_a, coord_b))| {
                vec![(coord_a, index), (coord_b, index)].into_iter()
            })
            .collect();

        for (index, &coordinate) in STANDARD_TILE_LOCATIONS.iter().enumerate() {
            let &(tile_type, roll_token) = &BALANCED_TILE_VALUES[index];
            board.tiles.insert(
                coordinate,
                InternalTileType::ResourceTile(tile_type),
            );

            if let Some(roll_token) = roll_token {
                board.roll_tokens.insert(coordinate, roll_token);
            }

            for neighbor in coordinate.neighbors() {
                if !board.tiles.contains_key(&neighbor) {

                    let building_tile =
                        if let Some(harbor_index) = harbor_buildings.get(&neighbor) {
                            BuildingTileContainer {
                                building: None,
                                harbor_type: Some((BALANCED_HARBOR_LOCATIONS[*harbor_index].1).0),
                            }
                        } else {
                            BuildingTileContainer {
                                building: None,
                                harbor_type: None,
                            }
                        };

                    board.tiles.insert(
                        neighbor,
                        InternalTileType::BuildingTile(building_tile),
                    );
                }
            }
        }

        board.harbors.extend(
            BALANCED_HARBOR_LOCATIONS.iter().cloned()
        );

        board
    }

    pub fn get_location(
        &self,
        coordinate: InternalCoord,
    ) -> (&InternalTileType, Option<&RollToken>) {
        let tile_type = self.tiles.get(&coordinate).expect(
            "Could not find tile for coordinate!",
        );
        let roll_token = self.roll_tokens.get(&coordinate);

        (tile_type, roll_token)
    }
}
