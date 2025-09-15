use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;

use crate::card::{Card, Suit};

#[derive(Resource, Default)]
pub struct CardAssets {
    pub primary: HashMap<String, Handle<Image>>,
    pub extra: HashMap<String, Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct GameInfo {
    pub _type: GameType,
    pub players: VecDeque<Player>,
    pub cards_placed: VecDeque<Card>,
    pub last_cards_placed: VecDeque<Card>,
    pub last_took: Option<usize>,
    pub trump: Option<Suit>,
    pub dealer: usize,
    pub round: usize,
    pub h_penalty: i32,
}

#[derive(Default, Debug)]
pub struct Player {
    pub name: String,
    pub cards: Vec<Card>,
    pub score: i32,
    pub called: i32,
    pub taken: i32,
}

#[derive(Default, Clone, Copy)]
pub enum GameType {
    // classic mode
    /*
       0   1
       1   2
       2   3
       3   4
       4   5
       5   6
       6   7
       7   8

       8   9
       9   9
       10  9
       11  9

       12  8
       13  7
       14  6
       15  5
       16  4
       17  3
       18  2
       19  1

       20  9
       21  9
       22  9
       23  9
    */
    #[default]
    Classic,

    // nines mode
    /*
       0   9
       1   9
       2   9
       3   9

       4   9
       5   9
       6   9
       7   9

       8   9
       9   9
       10  9
       11  9

       12  9
       13  9
       14  9
       15  9
    */
    Nines,
}

#[derive(Component)]
pub struct PlayerNode;

#[derive(Component)]
pub struct PlacedCardsNode;

#[derive(Component)]
pub struct PlacedCard;

#[derive(Component)]
pub struct ScoresText;

#[derive(Component)]
pub struct PlayerTag(pub String);
