use catan::board::{Board, InternalCoord, ResourceType};
use catan::common::Resource;
use std::collections::{HashMap, HashSet};
use rand::distributions::{IndependentSample, Range};
use rand::{Rng, ThreadRng, thread_rng};

pub struct CatanGame {
    board: Board,
    players: Vec<Player>,
    current_player_index: u32,
}

#[derive(Debug)]
pub struct Player {
    color: PlayerColor,
    resource_cards: HashMap<ResourceType, u32>,
    development_cards: HashMap<DevelopmentCardType, u32>,

    roads: Vec<(InternalCoord, InternalCoord)>,
    settlements: Vec<InternalCoord>,
    cities: Vec<InternalCoord>,
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

impl Resource for DevelopmentCardType {
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

impl Resource for BuildingType {
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
    _rng: ThreadRng
}

impl Dice {

    fn new() -> Dice {
        Dice {
            _range: Range::new(1, 6),
            _rng: thread_rng()
        }
    }

    fn roll(&mut self) -> u32 {
        self._range.ind_sample(&mut self._rng)
    }

}
