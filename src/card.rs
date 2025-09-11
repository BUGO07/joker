use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Card(pub String, pub usize); // card, player

impl Card {
    pub fn value(&self) -> usize {
        if self.0.starts_with("Joker") {
            return 15;
        }
        if self.0.ends_with("10") {
            return 10;
        }

        match &self.0[1..2] {
            "A" => 14,
            "K" => 13,
            "Q" => 12,
            "J" => 11,
            x => x.parse().unwrap(),
        }
    }
}
