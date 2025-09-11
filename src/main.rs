use std::collections::{HashMap, VecDeque};

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::card::Card;

pub mod card;

fn main() {
    App::new()
        .init_resource::<CardAssets>()
        .init_resource::<Players>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Joker Game".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, resize_event)
        .run();
}

#[derive(Resource, Default)]
pub struct CardAssets(HashMap<String, Handle<Image>>);

#[derive(Resource, Default)]
pub struct Players(VecDeque<(String, Vec<Card>)>);

const CARD_WIDTH: f32 = 290.0;
const CARD_HEIGHT: f32 = 400.0;
const CARD_SCALE: f32 = 1.0 / 5.0;
const CSW: f32 = CARD_WIDTH * CARD_SCALE;
const CSH: f32 = CARD_HEIGHT * CARD_SCALE;

fn setup(
    mut commands: Commands,
    mut assets: ResMut<CardAssets>,
    mut players: ResMut<Players>,
    asset_server: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    for asset in std::fs::read_dir("assets/cards").unwrap() {
        if let Ok(entry) = asset
            && let Ok(file_type) = entry.file_type()
            && file_type.is_file()
        {
            let name = entry
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned();

            assets.0.insert(
                name,
                asset_server.load(format!(
                    "cards/{}",
                    entry.file_name().to_string_lossy().into_owned()
                )),
            );
        }
    }

    let mut deck = assets.0.keys().cloned().collect::<Vec<_>>();

    for name in ["lela", "ilia", "lizi", "giorgi"] {
        players.0.push_back((name.to_string(), Vec::new()));
    }
    for (i, (_player, cards)) in players.0.iter_mut().enumerate() {
        for _ in 0..9 {
            let card = deck.remove(rand::random_range(0..deck.len()));
            cards.push(Card(card, i));
        }
        cards.sort_by_key(|x| std::cmp::Reverse(x.value()));
    }

    let current_player = "giorgi";
    while players.0[0].0 != current_player {
        let player = players.0.pop_front().unwrap();
        players.0.push_back(player)
    }
    let covered_card = asset_server.load("back/Red.png");

    for (i, (player, cards)) in players.0.iter().enumerate() {
        let mut player_node = commands.spawn((
            Visibility::Visible,
            Transform::from_rotation(Quat::from_rotation_z(i as f32 * -90f32.to_radians())),
        ));
        player_node.with_child((
            Text2d::new(player),
            Transform::from_xyz(
                0.0,
                // ahh
                if i % 2 == 0 {
                    (-window.height() + CSH) / 2.0 + CSH
                } else {
                    (-window.width() + CSW + player.len() as f32 * 20.0 * 1.2 * 0.5) / 2.0 + CSW
                },
                0.0,
            )
            .with_rotation(Quat::from_rotation_z(i as f32 * 90f32.to_radians())),
        ));

        for (j, card) in cards.iter().enumerate() {
            player_node.with_child((
                Sprite::from_image(if player == current_player {
                    assets.0[&card.0].clone()
                } else {
                    covered_card.clone()
                }),
                Transform::from_translation(if i % 2 == 0 {
                    vec3(
                        (j as f32 - (9.0 - 1.0) / 2.0) * CSW,
                        (-window.height() + CSH + 25.0) / 2.0,
                        0.0,
                    )
                } else {
                    vec3(
                        (j as f32 - (9.0 - 1.0) / 2.0) * CSW,
                        (-window.width() + CSW + 25.0) / 2.0,
                        0.0,
                    )
                })
                .with_scale(Vec3::ONE * CARD_SCALE),
                card.clone(),
            ));
        }
    }
}

fn resize_event(
    mut cards: Query<(&mut Transform, &Card), Without<Text2d>>,
    mut player_nametags: Query<(&mut Transform, &Text2d)>,
    mut resize_event: EventReader<WindowResized>,
    players: Res<Players>,
) {
    for event in resize_event.read() {
        for (mut transform, card) in cards.iter_mut() {
            transform.translation.y = if (3 - card.1) % 2 == 0 {
                (-event.height + CSH + 25.0) / 2.0
            } else {
                (-event.width + CSW + 25.0) / 2.0
            };
        }
        for (mut transform, text) in player_nametags.iter_mut() {
            let i = players
                .0
                .iter()
                .enumerate()
                .find(|(_, (name, _))| name == &text.0)
                .unwrap()
                .0;
            transform.translation.y = if i % 2 == 0 {
                (-event.height + CSH) / 2.0 + CSH
            } else {
                (-event.width + CSW + text.len() as f32 * 20.0 * 1.2 * 0.5) / 2.0 + CSW
            }
        }
    }
}
