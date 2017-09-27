use catan::board::{Board, InternalCoord, ResourceType};
use std::collections::HashMap;

pub struct CatanGame {
    board: Board,
    players: Vec<Player>,
    current_player_index: u32,
}

#[derive(Debug)]
pub struct Player {
    color: PlayerColor,
    resource_cards: HashMap<ResourceType, u32>,
    development_cards: HashMap<DevelopmentType, u32>,
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DevelopmentType {
    Knight,
    Progress(DevelopmentProgressType),
    VictoryPoint(DevelopmentVictoryPointType),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DevelopmentProgressType {
    RoadBuilding,
    Monopoly,
    YearOfPlenty,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DevelopmentVictoryPointType {
    Chapel,
    Library,
    GreatHall,
    Market,
    University,
}
