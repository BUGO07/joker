use std::collections::{HashMap, VecDeque};

use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use random_number::random;
// use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::card::Card;

pub mod card;

fn main() {
    App::new()
        .init_resource::<CardAssets>()
        .init_resource::<Players>()
        .init_resource::<CardsPlaced>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Joker Game".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        // .add_plugins((EguiPlugin::default(), WorldInspectorPlugin::default()))
        .add_systems(Startup, (load_assets, start_game).chain())
        .add_systems(
            Update,
            (cleanup, start_game)
                .chain()
                .run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(Update, award_scores)
        .add_systems(Update, resize_event)
        .run();
}

#[derive(Resource, Default)]
pub struct CardAssets(HashMap<String, Handle<Image>>);

#[derive(Resource, Default)]
pub struct Players(VecDeque<Player>);

#[derive(Default, Debug)]
pub struct Player {
    pub name: String,
    pub cards: Vec<Card>,
    pub score: i32,
    pub called: i32,
    pub taken: i32,
}

#[derive(Resource, Default, Debug)]
pub struct CardsPlaced(VecDeque<Card>);

#[derive(Component)]
pub struct PlayerNode;

#[derive(Component)]
pub struct PlacedCardsNode;

#[derive(Component)]
pub struct PlacedCard;

#[derive(Component)]
pub struct ScoresText;

const H_PENALTY: i32 = 200;

const CARD_WIDTH: f32 = 290.0;
const CARD_HEIGHT: f32 = 400.0;
const CARD_SCALE: f32 = 1.0 / 5.0;
const CSW: f32 = CARD_WIDTH * CARD_SCALE;
const CSH: f32 = CARD_HEIGHT * CARD_SCALE;
const ASSETS: &[&str] = &[
    "JokerRed",
    "JokerBlack",
    "S7",
    "S8",
    "S9",
    "S10",
    "SJ",
    "SQ",
    "SK",
    "SA",
    "D6",
    "D7",
    "D8",
    "D9",
    "D10",
    "DJ",
    "DQ",
    "DK",
    "DA",
    "C7",
    "C8",
    "C9",
    "C10",
    "CJ",
    "CQ",
    "CK",
    "CA",
    "H6",
    "H7",
    "H8",
    "H9",
    "H10",
    "HJ",
    "HQ",
    "HK",
    "HA",
];

fn load_assets(
    mut commands: Commands,
    mut assets: ResMut<CardAssets>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);
    for name in ASSETS.iter() {
        assets.0.insert(
            name.to_string(),
            asset_server.load(format!("cards/{}.png", name)),
        );
    }
}

fn start_game(
    mut commands: Commands,
    mut players: ResMut<Players>,
    assets: Res<CardAssets>,
    window: Single<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let mut deck = assets.0.keys().cloned().collect::<Vec<_>>();

    for name in ["lela", "ilia", "lizi", "giorgi"] {
        players.0.push_back(Player {
            name: name.to_string(),
            ..Default::default()
        });
    }

    let current_player = "giorgi";
    while players.0[0].name != current_player {
        let player = players.0.pop_front().unwrap();
        players.0.push_back(player)
    }

    for (i, player) in players.0.iter_mut().enumerate() {
        for _ in 0..9 {
            let card = deck.remove(random!(..deck.len()));
            player.cards.push(Card(card, i));
        }
        player.cards.sort_by_key(|x| std::cmp::Reverse(x.value()));
        player.cards.sort_by_key(|x| x.suit());
    }
    let covered_card = asset_server.load("back/Red.png");

    commands.spawn((Text::new("Scores:"), ScoresText));
    commands.spawn((Visibility::Visible, Transform::default(), PlacedCardsNode));

    for (i, player) in players.0.iter().enumerate() {
        let player_node = commands
            .spawn((
                Visibility::Visible,
                Transform::from_rotation(Quat::from_rotation_z(i as f32 * -90f32.to_radians())),
                PlayerNode,
            ))
            .id();
        commands.spawn((
            Text2d::new(&player.name),
            Transform::from_xyz(
                0.0,
                // ahh
                if i % 2 == 0 {
                    (-window.height() + CSH) / 2.0 + CSH
                } else {
                    (-window.width() + CSW + player.name.len() as f32 * 20.0 * 1.2 * 0.5) / 2.0
                        + CSW
                },
                0.0,
            )
            .with_rotation(Quat::from_rotation_z(i as f32 * 90f32.to_radians())),
            ChildOf(player_node),
        ));

        for (j, card) in player.cards.iter().enumerate() {
            commands
                .spawn((
                    Pickable::default(),
                    Sprite::from_image(if player.name == current_player {
                        assets.0[&card.0].clone()
                    } else {
                        covered_card.clone()
                    }),
                    Transform::from_xyz(
                        (j as f32 - (9.0 - 1.0) / 2.0) * CSW,
                        if i % 2 == 0 {
                            (-window.height() + CSH + 25.0) / 2.0
                        } else {
                            (-window.width() + CSW + 25.0) / 2.0
                        },
                        0.0,
                    )
                    .with_scale(Vec3::ONE * CARD_SCALE),
                    card.clone(),
                    ChildOf(player_node),
                ))
                .observe(
                    |trigger: Trigger<Pointer<Released>>,
                     mut cards: Query<(&mut Transform, &Card)>,
                     mut commands: Commands,
                     mut players: ResMut<Players>,
                     mut cards_placed: ResMut<CardsPlaced>,
                     assets: Res<CardAssets>,
                     placed_cards_node: Single<Entity, With<PlacedCardsNode>>| {
                        let (_, card) = cards.get(trigger.target).unwrap();

                        let pcards = &mut players.0[card.1].cards;
                        if let Some(first_card) = cards_placed.0.back() {
                            let card_suit = card.suit();
                            let first_suit = first_card.suit();
                            if pcards.iter().any(|x| x.suit() == first_suit)
                                && first_suit != 0
                                && card_suit != 0
                                && card_suit != first_suit
                            {
                                return;
                            }
                        }
                        cards_placed.0.push_front(
                            pcards.remove(pcards.iter().position(|x| x == card).unwrap()),
                        );
                        commands.entity(trigger.target).despawn();
                        let fc = cards_placed.0.back().unwrap();
                        commands.spawn((
                            Sprite::from_image(assets.0[&card.0].clone()),
                            Transform::from_translation(
                                match (cards_placed.0.len() - 1 + fc.1) % 4 {
                                    0 => Vec3::NEG_Y,
                                    1 => Vec3::NEG_X,
                                    2 => Vec3::Y,
                                    3 => Vec3::X,
                                    _ => unreachable!(),
                                } * CSH,
                            )
                            .with_rotation(Quat::from_rotation_z(
                                (cards_placed.0.len() - 1 + fc.1) as f32 * -90f32.to_radians(),
                            ))
                            .with_scale(Vec3::ONE * CARD_SCALE),
                            card.clone(),
                            PlacedCard,
                            ChildOf(*placed_cards_node),
                        ));
                        for (mut transform, c) in cards.iter_mut() {
                            // not my finest code
                            if let Some(pos) = pcards.iter().position(|x| x == c) {
                                transform.translation.x =
                                    (pos as f32 - (pcards.len() as f32 - 1.0) / 2.0) * CSW
                            }
                        }
                    },
                );
        }
    }
}

fn cleanup(
    mut commands: Commands,
    mut players: ResMut<Players>,
    mut cards_placed: ResMut<CardsPlaced>,
    player_nodes: Query<Entity, With<PlayerNode>>,
    placed_cards_node: Query<Entity, With<PlacedCardsNode>>,
    scores_text: Single<Entity, With<ScoresText>>,
) {
    for node in player_nodes {
        commands.entity(node).despawn();
    }
    for node in placed_cards_node {
        commands.entity(node).despawn();
    }
    commands.entity(*scores_text).despawn();

    cards_placed.0.clear();
    // TODO - TEMP
    players.0.clear();
}

fn award_scores(
    mut commands: Commands,
    mut cards_placed: ResMut<CardsPlaced>,
    mut players: ResMut<Players>,
    mut scores_text: Single<&mut Text, With<ScoresText>>,
    query: Query<(Entity, &Card), With<PlacedCard>>,
    cards_in_hand: Query<(Entity, &Card), Without<PlacedCard>>,
) {
    if cards_placed.0.len() == 4 {
        let mut cards = query.iter().collect::<Vec<_>>();
        cards.sort_by_key(|(_, card)| std::cmp::Reverse(card.value()));
        let player = &mut players.0[cards[0].1.1];
        player.taken += 1;
        for (entity, _) in cards {
            commands.entity(entity).despawn();
        }
        cards_placed.0.clear();
    }
    // round over
    if !players.0.is_empty() && cards_in_hand.is_empty() {
        let mut text = String::new();
        for player in players.0.iter_mut() {
            if player.taken == player.called {
                player.score += (1 + player.taken) * 50
            } else if player.taken > 0 {
                player.score += player.taken * 10
            } else {
                player.score -= H_PENALTY;
            }
            text.push_str(&format!(
                "{}, {}/{} | {}\n",
                player.name, player.taken, player.called, player.score
            ));
        }
        scores_text.0 = text;
        // TODO - TEMP
        commands.run_system_cached(cleanup);
        commands.run_system_cached(start_game);
    }
}

#[allow(clippy::type_complexity)]
fn resize_event(
    mut cards: Query<(&mut Transform, &Card), (Without<Text2d>, Without<PlacedCard>)>,
    mut player_nametags: Query<(&mut Transform, &Text2d)>,
    mut resize_event: EventReader<WindowResized>,
    players: Res<Players>,
) {
    for event in resize_event.read() {
        for (mut transform, card) in cards.iter_mut() {
            transform.translation.y = if card.1 % 2 == 0 {
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
                .find(|(_, player)| player.name == text.0)
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
