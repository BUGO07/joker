use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Joker,
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

#[derive(Component, Clone, PartialEq, Debug)]
pub struct Card(pub String, pub usize); // card, player

impl Card {
    pub fn suit(&self) -> Suit {
        match &self.0[0..1] {
            "J" => Suit::Joker,
            "S" => Suit::Spades,
            "D" => Suit::Diamonds,
            "C" => Suit::Clubs,
            "H" => Suit::Hearts,
            _ => unreachable!(),
        }
    }

    pub fn value(&self, trump: Option<Suit>) -> usize {
        if self.0.starts_with("J") {
            return usize::MAX;
        }

        let value = if self.0.ends_with("10") {
            10
        } else {
            match &self.0[1..2] {
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => 11,
                x => x.parse().unwrap(),
            }
        };

        if let Some(t) = trump
            && self.suit() == t
        {
            value + 15
        } else {
            value
        }
    }
}
