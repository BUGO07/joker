use bevy::prelude::*;

use crate::GameInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Joker(bool),
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Rank {
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker = u8::MAX,
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub player: usize,
}

impl Card {
    pub fn from_string(string: String, player: usize) -> Self {
        Self {
            suit: match &string[0..1] {
                "J" => match &string[1..2] {
                    "R" => Suit::Joker(false),
                    "B" => Suit::Joker(true),
                    _ => unreachable!(),
                },
                "S" => Suit::Spades,
                "D" => Suit::Diamonds,
                "C" => Suit::Clubs,
                "H" => Suit::Hearts,
                _ => unreachable!(),
            },
            rank: match &string[1..2] {
                "R" | "B" => Rank::Joker,
                "A" => Rank::Ace,
                "K" => Rank::King,
                "Q" => Rank::Queen,
                "J" => Rank::Jack,
                "1" => Rank::Ten,
                "9" => Rank::Nine,
                "8" => Rank::Eight,
                "7" => Rank::Seven,
                "6" => Rank::Six,
                _ => unreachable!(),
            },
            player,
        }
    }

    pub fn as_string(&self) -> String {
        let mut string = String::new();

        string.push(match self.suit {
            Suit::Joker(black) => return (if black { "JB" } else { "JR" }).to_string(),
            Suit::Spades => 'S',
            Suit::Diamonds => 'D',
            Suit::Clubs => 'C',
            Suit::Hearts => 'H',
        });

        string.push(match self.rank {
            Rank::Ace => 'A',
            Rank::King => 'K',
            Rank::Queen => 'Q',
            Rank::Jack => 'J',
            Rank::Ten => '1',
            Rank::Nine => '9',
            Rank::Eight => '8',
            Rank::Seven => '7',
            Rank::Six => '6',
            _ => unreachable!(),
        });

        string
    }

    pub fn value(&self, trump: Option<Suit>) -> u8 {
        if self.rank != Rank::Joker
            && let Some(t) = trump
            && self.suit == t
        {
            self.rank as u8 + 10
        } else {
            self.rank as u8
        }
    }

    pub fn can_place(&self, game_info: &GameInfo) -> bool {
        let player = &game_info.players[self.player];

        // check that it's the player's turn to place
        /*
             if this is not the first card placed, check that the player index is last_player + 1
             if this is the first card placed:
                check that the player placing the card is the same one that took last cards
                check that the player index is dealer + 1
        */
        if let Some(last_card) = game_info.cards_placed.front() {
            if player.name != game_info.players[(last_card.player + 1) % 4].name {
                return false;
            }
        } else if let Some(x) = &game_info.last_took {
            if x != &player.name {
                return false;
            }
        } else if player.name != game_info.players[(game_info.dealer + 1) % 4].name {
            return false;
        }

        // check the first card and make sure:
        // either:
        /*
            the suit is the same as the card being placed
            the first card is a joker
            the player's card is a joker
        */
        // if the player doesn't have a same suit of card,
        // and they don't have a trump, they can place any card
        if let Some(first_card) = game_info.cards_placed.back() {
            let card_suit = self.suit;
            let first_suit = first_card.suit;

            // the player has already placed a card
            if game_info
                .cards_placed
                .iter()
                .any(|card| card.player == self.player)
            {
                return false;
            }

            if player.cards.iter().any(|card| card.suit == first_suit)
                && card_suit != first_suit
                && !(matches!(first_suit, Suit::Joker(_)) || matches!(card_suit, Suit::Joker(_)))
            {
                return false;
            }

            // TODO: don't place other cards if the player has a trump
        }

        true
    }
}
