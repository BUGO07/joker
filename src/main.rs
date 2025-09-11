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
            start_game.run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(Update, resize_event)
        .run();
}

#[derive(Resource, Default)]
pub struct CardAssets(HashMap<String, Handle<Image>>);

#[derive(Resource, Default)]
pub struct Players(VecDeque<(String, Vec<Card>)>);

#[derive(Component)]
pub struct PlayerNode;

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
    player_nodes: Query<Entity, With<PlayerNode>>,
) {
    for node in player_nodes {
        commands.entity(node).despawn();
    }
    players.0.clear();
    let mut deck = assets.0.keys().cloned().collect::<Vec<_>>();

    for name in ["lela", "ilia", "lizi", "giorgi"] {
        players.0.push_back((name.to_string(), Vec::new()));
    }

    let current_player = "giorgi";
    while players.0[0].0 != current_player {
        let player = players.0.pop_front().unwrap();
        players.0.push_back(player)
    }

    for (i, (_player, cards)) in players.0.iter_mut().enumerate() {
        for _ in 0..9 {
            let card = deck.remove(random!(..deck.len()));
            cards.push(Card(card, i));
        }
        cards.sort_by_key(|x| std::cmp::Reverse(x.value()));
        cards.sort_by_key(|x| x.suit());
    }
    let covered_card = asset_server.load("back/Red.png");

    for (i, (player, cards)) in players.0.iter().enumerate() {
        let player_node = commands
            .spawn((
                Visibility::Visible,
                Transform::from_rotation(Quat::from_rotation_z(i as f32 * -90f32.to_radians())),
                PlayerNode,
            ))
            .id();
        commands.spawn((
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
            ChildOf(player_node),
        ));

        for (j, card) in cards.iter().enumerate() {
            commands
                .spawn((
                    Pickable::default(),
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
                    ChildOf(player_node),
                ))
                .observe(
                    |trigger: Trigger<Pointer<Released>>,
                     cards: Query<&Card>,
                     players: Res<Players>| {
                        let card = cards.get(trigger.target).unwrap();
                        info!("card - {} | player - {}", card.0, players.0[card.1].0)
                    },
                );
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
