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

#[derive(Default, Debug)]
pub struct Player {
    pub name: String,
    pub cards: Vec<Card>,
    pub score: i32,
    pub called: i32,
    pub taken: i32,
}

#[derive(Default)]
pub enum GameType {
    Classic,
    #[default]
    Nines,
}

#[derive(Resource, Default)]
pub struct GameInfo {
    _type: GameType,
    players: VecDeque<Player>,
    cards_placed: VecDeque<Card>,
    last_cards_placed: VecDeque<Card>,
    last_took: Option<String>, // player name
    dealer: usize,
}

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
    assets: Res<CardAssets>,
    window: Single<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let mut deck = assets.0.keys().cloned().collect::<Vec<_>>();
    let mut game_info = GameInfo::default();

    for name in ["lela", "ilia", "lizi", "giorgi"] {
        game_info.players.push_back(Player {
            name: name.to_string(),
            ..Default::default()
        });
    }

    let current_player = "giorgi";
    while game_info.players[0].name != current_player {
        let player = game_info.players.pop_front().unwrap();
        game_info.players.push_back(player)
    }

    for (i, player) in game_info.players.iter_mut().enumerate() {
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

    for (i, player) in game_info.players.iter().enumerate() {
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
                     mut game_info: ResMut<GameInfo>,
                     assets: Res<CardAssets>,
                     placed_cards_node: Single<Entity, With<PlacedCardsNode>>| {
                        let (_, card) = cards.get(trigger.target).unwrap();

                        let GameInfo {
                            players,
                            cards_placed,
                            _type: _,
                            last_cards_placed: _,
                            last_took,
                            dealer,
                        } = &mut *game_info;
                        let player = &players[card.1];
                        if let Some(first_card) = cards_placed.back() {
                            let card_suit = card.suit();
                            let first_suit = first_card.suit();
                            if cards_placed.iter().any(|x| x.1 == card.1)
                                || (player.cards.iter().any(|x| x.suit() == first_suit)
                                    && first_suit != 0
                                    && card_suit != 0
                                    && card_suit != first_suit)
                            {
                                return;
                            }
                        } else if let Some(x) = last_took {
                            if *x != player.name {
                                return;
                            }
                        } else if player.name != players[*dealer + 1].name {
                            return;
                        }

                        let pcards = &mut players[card.1].cards;
                        cards_placed.push_front(
                            pcards.remove(pcards.iter().position(|x| x == card).unwrap()),
                        );
                        commands.entity(trigger.target).despawn();
                        let fc = cards_placed.back().unwrap();
                        commands.spawn((
                            Sprite::from_image(assets.0[&card.0].clone()),
                            Transform::from_translation(
                                match (cards_placed.len() - 1 + fc.1) % 4 {
                                    0 => Vec3::NEG_Y,
                                    1 => Vec3::NEG_X,
                                    2 => Vec3::Y,
                                    3 => Vec3::X,
                                    _ => unreachable!(),
                                } * CSH,
                            )
                            .with_rotation(Quat::from_rotation_z(
                                (cards_placed.len() - 1 + fc.1) as f32 * -90f32.to_radians(),
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
    commands.insert_resource(game_info);
}

fn cleanup(
    mut commands: Commands,
    mut game_info: ResMut<GameInfo>,
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

    game_info.cards_placed.clear();
    // TODO - TEMP
    game_info.players.clear();
}

fn award_scores(
    mut commands: Commands,
    mut game_info: ResMut<GameInfo>,
    mut scores_text: Single<&mut Text, With<ScoresText>>,
    query: Query<(Entity, &Card), With<PlacedCard>>,
    cards_in_hand: Query<(Entity, &Card), Without<PlacedCard>>,
) {
    if game_info.cards_placed.len() == 4 {
        let mut cards = query.iter().collect::<Vec<_>>();
        cards.sort_by_key(|(_, card)| std::cmp::Reverse(card.value()));
        let player = &mut game_info.players[cards[0].1.1];
        player.taken += 1;
        game_info.last_took = Some(player.name.clone());
        game_info.last_cards_placed = game_info.cards_placed.clone();
        for (entity, _) in cards {
            commands.entity(entity).despawn();
        }
        game_info.cards_placed.clear();
    }
    // round over
    if !game_info.players.is_empty() && cards_in_hand.is_empty() {
        let mut text = String::new();
        for player in game_info.players.iter_mut() {
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
    game_info: Res<GameInfo>,
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
            let i = game_info
                .players
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
