use bevy::{platform::collections::HashMap, prelude::*, window::PrimaryWindow};

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
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource, Default)]
pub struct CardAssets(HashMap<String, Handle<Image>>);

const CARD_WIDTH: f32 = 290.0;
const CARD_HEIGHT: f32 = 400.0;
const CARD_SCALE: f32 = 1.0 / 5.0;
const CSW: f32 = CARD_WIDTH * CARD_SCALE;
const CSH: f32 = CARD_HEIGHT * CARD_SCALE;

fn setup(
    mut commands: Commands,
    mut assets: ResMut<CardAssets>,
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

    let mut players = Vec::new();
    for name in ["lela", "ilia", "lizi", "giorgi"] {
        players.push((name, Vec::new()));
    }
    for (_player, cards) in players.iter_mut() {
        for _ in 0..9 {
            let card = deck.remove(rand::random_range(0..deck.len()));
            cards.push(Card(card));
        }
        cards.sort_by_key(|x| std::cmp::Reverse(x.value()));
    }

    let current_player = "giorgi";
    let covered_card = asset_server.load("back/Red.png");

    for (i, (player, cards)) in players.iter().enumerate() {
        for (j, card) in cards.iter().enumerate() {
            commands
                .spawn((
                    Visibility::Visible,
                    Transform::from_rotation(Quat::from_rotation_z(
                        (3 - i) as f32 * 90f32.to_radians(),
                    )),
                ))
                .with_child((
                    Sprite::from_image(if player == &current_player {
                        assets.0[&card.0].clone()
                    } else {
                        covered_card.clone()
                    }),
                    Transform::from_xyz(
                        (j as f32 - (9.0 - 1.0) / 2.0) * CSW,
                        (-window.height() + CSH + 25.0) / 2.0,
                        0.0,
                    )
                    .with_scale(Vec3::ONE * CARD_SCALE),
                ));
        }
    }
}
