use catan::board::{Board, InternalCoord};
use catan::common::GameResource;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub, Index};
use rand::distributions::{IndependentSample, Range};
use rand::{Rng, ThreadRng, thread_rng};

pub struct CatanGame {
    board: Board,
    players: Vec<Player>,
    current_player_index: u32,
    dice: [Dice; 2],
}

#[derive(Debug)]
pub struct Player {
    color: PlayerColor,
    resources: ResourceCollection,
    development_cards: HashMap<DevelopmentCardType, u32>,
    buildings: HashMap<InternalCoord, BuildingType>,
    roads: HashSet<(InternalCoord, InternalCoord)>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PlayerColor {
    Red,
    White,
    Orange,
    Blue,
}

pub enum PlayerAction {
    Roll(u32),
    Build(BuildingType),
    PurchaseDevelopmentCard(DevelopmentCardType),
    PlayDevelopmentCard(DevelopmentCardType),
    TradeResources(),
}

pub struct PlayerTrade<'a> {
    offering_player: &'a Player,
    accepting_player: &'a Player,
    offer: Vec<ResourceType>,
    receipt: Vec<ResourceType>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum DevelopmentCardType {
    Knight,
    Progress(DevelopmentProgressType),
    VictoryPoint(DevelopmentVictoryPointType),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum DevelopmentProgressType {
    RoadBuilding,
    Monopoly,
    YearOfPlenty,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum DevelopmentVictoryPointType {
    Chapel,
    Library,
    GreatHall,
    Market,
    University,
}

impl GameResource for DevelopmentCardType {
    fn count(self) -> usize {
        match self {
            DevelopmentCardType::Knight => 14,
            DevelopmentCardType::Progress(DevelopmentProgressType::RoadBuilding) => 2,
            DevelopmentCardType::Progress(DevelopmentProgressType::Monopoly) => 2,
            DevelopmentCardType::Progress(DevelopmentProgressType::YearOfPlenty) => 2,
            DevelopmentCardType::VictoryPoint(DevelopmentVictoryPointType::Chapel) => 1,
            DevelopmentCardType::VictoryPoint(DevelopmentVictoryPointType::Library) => 1,
            DevelopmentCardType::VictoryPoint(DevelopmentVictoryPointType::GreatHall) => 1,
            DevelopmentCardType::VictoryPoint(DevelopmentVictoryPointType::Market) => 1,
            DevelopmentCardType::VictoryPoint(DevelopmentVictoryPointType::University) => 1,
        }
    }

    fn all_variants() -> HashSet<DevelopmentCardType> {
        let mut variants: HashSet<DevelopmentCardType> = HashSet::new();

        variants.insert(DevelopmentCardType::Knight);
        variants.insert(DevelopmentCardType::Progress(
            DevelopmentProgressType::RoadBuilding,
        ));
        variants.insert(DevelopmentCardType::Progress(
            DevelopmentProgressType::Monopoly,
        ));
        variants.insert(DevelopmentCardType::Progress(
            DevelopmentProgressType::YearOfPlenty,
        ));
        variants.insert(DevelopmentCardType::VictoryPoint(
            DevelopmentVictoryPointType::Chapel,
        ));
        variants.insert(DevelopmentCardType::VictoryPoint(
            DevelopmentVictoryPointType::Library,
        ));
        variants.insert(DevelopmentCardType::VictoryPoint(
            DevelopmentVictoryPointType::GreatHall,
        ));
        variants.insert(DevelopmentCardType::VictoryPoint(
            DevelopmentVictoryPointType::Market,
        ));
        variants.insert(DevelopmentCardType::VictoryPoint(
            DevelopmentVictoryPointType::University,
        ));

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

#[derive(Debug)]
pub struct Dice {
    _range: Range<u32>,
    _rng: ThreadRng,
}

impl Dice {
    fn new() -> Dice {
        Dice {
            _range: Range::new(1, 7),
            _rng: thread_rng(),
        }
    }

    fn roll(&mut self) -> u32 {
        self._range.ind_sample(&mut self._rng)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ResourceType {
    Ore,
    Brick,
    Grain,
    Wool,
    Lumber,
}

impl GameResource for ResourceType {
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

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ResourceCollection {
    ore: u32,
    brick: u32,
    grain: u32,
    wool: u32,
    lumber: u32,
}

impl ResourceCollection {
    pub fn new(ore: u32, brick: u32, grain: u32, wool: u32, lumber: u32) -> ResourceCollection {
        ResourceCollection {
            ore,
            brick,
            grain,
            wool,
            lumber,
        }
    }
}

impl Add for ResourceCollection {
    type Output = ResourceCollection;

    fn add(self, other: ResourceCollection) -> ResourceCollection {
        ResourceCollection {
            ore: self.ore + other.ore,
            brick: self.brick + other.brick,
            grain: self.grain + other.grain,
            wool: self.wool + other.wool,
            lumber: self.lumber + other.lumber,
        }
    }
}

impl Sub for ResourceCollection {
    type Output = ResourceCollection;

    fn sub(self, other: ResourceCollection) -> ResourceCollection {
        ResourceCollection {
            ore: self.ore - other.ore,
            brick: self.brick - other.brick,
            grain: self.grain - other.grain,
            wool: self.wool - other.wool,
            lumber: self.lumber - other.lumber,
        }
    }
}

impl Index<ResourceType> for ResourceCollection {
    type Output = u32;

    fn index(&self, index: ResourceType) -> &u32 {
        match index {
            ResourceType::Ore => &self.ore,
            ResourceType::Brick => &self.brick,
            ResourceType::Grain => &self.grain,
            ResourceType::Wool => &self.wool,
            ResourceType::Lumber => &self.lumber,            
        }
    }
}
