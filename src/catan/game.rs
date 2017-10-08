use catan::board::{Board, InternalCoord};
use catan::common::GameResource;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub, Index, IndexMut};
use std::fmt;
use std::cmp::Ordering;
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

pub enum PlayerAction<'a> {
    Roll(u32),
    Build(BuildingType),
    PurchaseDevelopmentCard(DevelopmentCardType),
    PlayDevelopmentCard(DevelopmentCardType),
    TradeResources(PlayerTrade<'a>),
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

#[derive(PartialEq, Eq)]
pub struct ResourceCollection {
    ore: u32,
    brick: u32,
    grain: u32,
    wool: u32,
    lumber: u32,
}

impl PartialOrd for ResourceCollection {
    fn partial_cmp(&self, other: &ResourceCollection) -> Option<Ordering> {
        if self.ore > other.ore && self.brick > other.brick && self.grain > other.grain &&
            self.wool > other.wool && self.lumber > other.lumber
        {
            Some(Ordering::Greater)
        } else if self.ore < other.ore && self.brick < other.brick &&
                   self.grain < other.grain && self.wool < other.wool &&
                   self.lumber < other.lumber
        {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
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

    pub fn satisfies(&self, other: &ResourceCollection) -> bool {
        self.ore >= other.ore && self.brick >= other.brick && self.grain >= other.grain &&
            self.wool >= other.wool && self.lumber >= other.lumber
    }
}

impl fmt::Debug for ResourceCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Ore: {}, Brick: {}, Grain: {}, Wool: {}, Lumber: {})",
            self.ore,
            self.brick,
            self.grain,
            self.wool,
            self.lumber
        )
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

impl IndexMut<ResourceType> for ResourceCollection {
    fn index_mut(&mut self, index: ResourceType) -> &mut Self::Output {
        match index {
            ResourceType::Ore => &mut self.ore,
            ResourceType::Brick => &mut self.brick,
            ResourceType::Grain => &mut self.grain,
            ResourceType::Wool => &mut self.wool,
            ResourceType::Lumber => &mut self.lumber,            
        }
    }
}

#[cfg(test)]
mod resource_collection_tests {
    use catan::game::ResourceCollection;
    use catan::game::ResourceType;

    #[test]
    fn test_creation() {
        let resource_collection = ResourceCollection::new(3, 2, 1, 0, 4);

        assert_eq!(resource_collection.ore, 3);
        assert_eq!(resource_collection.brick, 2);
        assert_eq!(resource_collection.grain, 1);
        assert_eq!(resource_collection.wool, 0);
        assert_eq!(resource_collection.lumber, 4);
    }

    #[test]
    fn test_satisfies() {
        let collection_a = ResourceCollection::new(3, 2, 1, 0, 4);
        let does_satisfy = ResourceCollection::new(3, 1, 0, 0, 2);
        let does_not_satisfy = ResourceCollection::new(2, 2, 2, 2, 2);

        assert!(collection_a.satisfies(&does_satisfy));
        assert!(!collection_a.satisfies(&does_not_satisfy));
    }

    #[test]
    fn test_equality() {
        let equal_a = ResourceCollection::new(1, 2, 1, 1, 1);
        let equal_b = ResourceCollection::new(1, 2, 1, 1, 1);
        let not_equal = ResourceCollection::new(1, 1, 1, 2, 1);

        assert!(equal_a == equal_b);
        assert!(equal_a != not_equal);
        assert!(equal_b != not_equal);
    }

    #[test]
    fn test_comparison() {
        let lower_collection = ResourceCollection::new(0, 1, 2, 3, 4);
        let middle_collection = ResourceCollection::new(1, 2, 3, 4, 5);
        let upper_collection = ResourceCollection::new(2, 3, 4, 5, 6);

        let lower_middle_collection = ResourceCollection::new(0, 1, 2, 4, 5);
        let middle_upper_collection = ResourceCollection::new(1, 2, 3, 5, 6);

        // Normal orderings
        assert!(lower_collection < middle_collection);
        assert!(middle_collection < upper_collection);
        assert!(lower_collection < middle_collection);

        // Incomplete orderings, no order can be produced so everything is false
        assert!(!(lower_collection < lower_middle_collection));
        assert!(!(lower_collection > lower_middle_collection));
        assert!(!(lower_collection == lower_middle_collection));

        assert!(!(lower_middle_collection < middle_collection));
        assert!(!(lower_middle_collection > middle_collection));
        assert!(!(lower_middle_collection == middle_collection));

        assert!(!(middle_collection < middle_upper_collection));
        assert!(!(middle_collection > middle_upper_collection));
        assert!(!(middle_collection == middle_upper_collection));

        assert!(!(middle_upper_collection < upper_collection));
        assert!(!(middle_upper_collection > upper_collection));
        assert!(!(middle_upper_collection == upper_collection));
    }

    #[test]
    fn test_addition() {
        let collection_a = ResourceCollection::new(0, 0, 1, 1, 1);
        let collection_b = ResourceCollection::new(2, 1, 0, 1, 2);
        let result = ResourceCollection::new(2, 1, 1, 2, 3);

        assert_eq!(collection_a + collection_b, result);
    }

    #[test]
    fn test_subtraction() {
        let collection_a = ResourceCollection::new(2, 1, 1, 2, 3);
        let collection_b = ResourceCollection::new(2, 1, 0, 1, 2);
        let result = ResourceCollection::new(0, 0, 1, 1, 1);

        assert_eq!(collection_a - collection_b, result);
    }

    #[test]
    fn test_indexing() {
        let collection = ResourceCollection::new(2, 3, 5, 7, 9);

        assert_eq!(collection[ResourceType::Ore], 2);
        assert_eq!(collection[ResourceType::Brick], 3);
        assert_eq!(collection[ResourceType::Grain], 5);
        assert_eq!(collection[ResourceType::Wool], 7);
        assert_eq!(collection[ResourceType::Lumber], 9);
    }

    #[test]
    fn test_mutable_indexing() {
        let mut collection = ResourceCollection::new(2, 3, 5, 7, 9);

        assert_eq!(collection[ResourceType::Ore], 2);
        assert_eq!(collection[ResourceType::Brick], 3);
        assert_eq!(collection[ResourceType::Grain], 5);
        assert_eq!(collection[ResourceType::Wool], 7);
        assert_eq!(collection[ResourceType::Lumber], 9);

        collection[ResourceType::Grain] -= 2;
        collection[ResourceType::Ore] += 3;
        collection[ResourceType::Lumber] /= 3;
        collection[ResourceType::Wool] %= 3;
        collection[ResourceType::Brick] = 0;

        assert_eq!(collection[ResourceType::Ore], 5);
        assert_eq!(collection[ResourceType::Brick], 0);
        assert_eq!(collection[ResourceType::Grain], 3);
        assert_eq!(collection[ResourceType::Wool], 1);
        assert_eq!(collection[ResourceType::Lumber], 3);
    }
}
