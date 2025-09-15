use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use random_number::random;
// use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{card::*, components::*, consts::*};

mod card;
mod components;
mod consts;

fn main() {
    App::new()
        .init_resource::<CardAssets>()
        .init_resource::<GameInfo>()
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
        .add_systems(
            Update,
            (card_highlight, update_nametags).run_if(resource_changed::<GameInfo>),
        )
        .add_systems(Update, resize_event)
        .run();
}

fn load_assets(
    mut commands: Commands,
    mut assets: ResMut<CardAssets>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);
    for name in ASSETS.iter() {
        assets.primary.insert(
            name.to_string(),
            asset_server.load(format!("cards/{}.png", name)),
        );
    }
    assets
        .extra
        .insert("back".to_string(), asset_server.load("back/R.png"));
}

fn start_game(mut commands: Commands, mut game_info: ResMut<GameInfo>) {
    game_info.dealer = 3;
    game_info.h_penalty = 200;

    for name in ["lela", "ilia", "lizi", "giorgi"] {
        game_info.players.push_back(Player {
            name: name.to_string(),
            ..Default::default()
        });
    }

    commands.spawn((Text::new("Scores:"), ScoresText));
    commands.spawn((Visibility::Visible, Transform::default(), PlacedCardsNode));

    commands.run_system_cached(start_round);
}

fn start_round(
    mut commands: Commands,
    mut game_info: ResMut<GameInfo>,
    assets: Res<CardAssets>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let mut deck = assets.primary.keys().cloned().collect::<Vec<_>>();

    let current_player = "giorgi";
    while game_info.players[0].name != current_player {
        let player = game_info.players.pop_front().unwrap();
        game_info.players.push_back(player)
    }

    game_info.trump = Some(Card::from_string(deck[random!(..deck.len())].clone(), usize::MAX).suit);

    let _type = game_info._type;
    let round = game_info.round;
    let trump = game_info.trump;

    for (i, player) in game_info.players.iter_mut().enumerate() {
        for _ in if matches!(_type, GameType::Nines) {
            0..9
        } else if (0..8).contains(&round) {
            0..round + 1
        } else if (12..20).contains(&round) {
            0..(8 - (round - 12)) + 1
        } else {
            0..9
        } {
            let card = deck.remove(random!(..deck.len()));
            player.cards.push(Card::from_string(card, i));
        }
        player
            .cards
            .sort_by_key(|x| std::cmp::Reverse(x.value(trump)));
        player.cards.sort_by_key(|x| x.suit);
    }

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
            TextLayout::new_with_justify(JustifyText::Center),
            PlayerTag(player.name.clone()),
            Transform::from_rotation(Quat::from_rotation_z(i as f32 * 90f32.to_radians())),
            ChildOf(player_node),
        ));

        for (j, card) in player.cards.iter().enumerate() {
            commands
                .spawn((
                    Pickable::default(),
                    Sprite::from_image(if player.name == current_player {
                        assets.primary[&card.as_string()].clone()
                    } else {
                        // assets.extra["back"].clone()
                        assets.primary[&card.as_string()].clone()
                    }),
                    Transform::from_xyz(
                        (j as f32 - (player.cards.len() as f32 - 1.0) / 2.0) * CSW,
                        if i % 2 == 0 {
                            (-window.height() + CSH + 25.0) / 2.0
                        } else {
                            (-window.width() + CSW + 25.0) / 2.0
                        },
                        0.0,
                    )
                    .with_scale(Vec3::ONE * CARD_SCALE),
                    *card,
                    ChildOf(player_node),
                ))
                .observe(
                    |trigger: Trigger<Pointer<Over>>,
                     mut cards: Query<(&mut Transform, &Card), Without<PlacedCard>>,
                     game_info: Res<GameInfo>| {
                        let (mut transform, card) = cards.get_mut(trigger.target).unwrap();

                        if !card.can_place(&game_info) {
                            return;
                        }

                        // if card.player == 0 {
                        transform.translation.y += 7.5;
                        transform.scale.y *= 1.2;
                        // }
                    },
                )
                .observe(
                    |trigger: Trigger<Pointer<Out>>,
                     mut cards: Query<(&mut Transform, &Card), Without<PlacedCard>>,
                     game_info: Res<GameInfo>| {
                        let (mut transform, card) = cards.get_mut(trigger.target).unwrap();

                        if !card.can_place(&game_info) {
                            return;
                        }

                        // if card.player == 0 {
                        transform.translation.y -= 7.5;
                        transform.scale.y /= 1.2;
                        // }
                    },
                )
                .observe(
                    |trigger: Trigger<Pointer<Released>>,
                     mut cards: Query<(&mut Transform, &Card), Without<PlacedCard>>,
                     mut commands: Commands,
                     mut game_info: ResMut<GameInfo>,
                     assets: Res<CardAssets>,
                     placed_cards_node: Single<Entity, With<PlacedCardsNode>>| {
                        let (_, card) = cards.get(trigger.target).unwrap();

                        if !card.can_place(&game_info) {
                            return;
                        }

                        {
                            let GameInfo {
                                players,
                                cards_placed,
                                ..
                            } = &mut *game_info;

                            let pcards = &mut players[card.player].cards;
                            cards_placed.push_front(
                                pcards.remove(pcards.iter().position(|x| x == card).unwrap()),
                            );
                            commands.entity(trigger.target).despawn();
                            let fc = cards_placed.back().unwrap();
                            commands.spawn((
                                Sprite::from_image(assets.primary[&card.as_string()].clone()),
                                Transform::from_translation(
                                    match (cards_placed.len() - 1 + fc.player) % 4 {
                                        0 => Vec3::NEG_Y,
                                        1 => Vec3::NEG_X,
                                        2 => Vec3::Y,
                                        3 => Vec3::X,
                                        _ => unreachable!(),
                                    } * CSH,
                                )
                                .with_rotation(Quat::from_rotation_z(
                                    (cards_placed.len() - 1 + fc.player) as f32
                                        * -90f32.to_radians(),
                                ))
                                .with_scale(Vec3::ONE * CARD_SCALE),
                                *card,
                                PlacedCard,
                                ChildOf(*placed_cards_node),
                            ));
                        }

                        let pcards = &game_info.players[card.player].cards;
                        for (mut transform, card) in cards.iter_mut() {
                            // not my finest code
                            if let Some(pos) = pcards.iter().position(|c| c == card) {
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

    commands.insert_resource(GameInfo::default());
}

fn award_scores(
    mut commands: Commands,
    mut game_info: ResMut<GameInfo>,
    mut scores_text: Single<&mut Text, With<ScoresText>>,
    query: Query<Entity, With<PlacedCard>>,
    cards_in_hand: Query<(Entity, &Card), Without<PlacedCard>>,
) {
    if game_info.cards_placed.len() == 4 {
        let mut cards = game_info.cards_placed.iter().copied().collect::<Vec<_>>();
        cards.sort_by_key(|card| std::cmp::Reverse(card.value(game_info.trump)));
        let player = &mut game_info.players[cards[0].player];
        player.taken += 1;
        game_info.last_took = Some(cards[0].player);
        game_info.last_cards_placed = game_info.cards_placed.clone();
        for entity in query {
            commands.entity(entity).despawn();
        }
        game_info.cards_placed.clear();
    }

    // round over
    if !game_info.players.is_empty() && cards_in_hand.is_empty() {
        let h_penalty = game_info.h_penalty;
        for player in game_info.players.iter_mut() {
            if player.taken == player.called {
                player.score += (1 + player.taken) * 50
            } else if player.taken == 0 {
                player.score -= h_penalty;
            } else {
                player.score += player.taken * 10
            }
            player.taken = 0;
            player.called = 0;
            println!(
                "{}, {}/{} | {}",
                player.name, player.taken, player.called, player.score
            );
        }
        game_info.dealer = (game_info.dealer + 1) % 4;
        game_info.round += 1;
        game_info.last_took = None;
        game_info.last_cards_placed.clear();

        // game over
        if game_info.round
            == match game_info._type {
                GameType::Classic => 24,
                GameType::Nines => 16,
            }
        {
            // TODO - game over screen, show scores
            commands.run_system_cached(cleanup);
            commands.run_system_cached(start_game);
        } else {
            commands.run_system_cached(start_round);
        }
    }

    // TODO - TEMP
    scores_text.0 = format!(
        "Round: {}\nTrump: {:?}",
        game_info.round,
        game_info.trump.unwrap_or(Suit::Joker(false))
    );
}

#[allow(clippy::type_complexity)]
fn card_highlight(
    mut cards: Query<(&mut Sprite, &Card), Without<PlacedCard>>,
    game_info: Res<GameInfo>,
) {
    for (mut sprite, card) in cards.iter_mut() {
        // if card.player == 0 {
        sprite.color = if card.can_place(&game_info) {
            Color::srgb(1.0, 1.0, 1.0)
        } else {
            Color::srgb(0.5, 0.5, 0.5)
        }
        // }
    }
}

fn resize_event(
    mut commands: Commands,
    mut cards: Query<(&mut Transform, &Card), Without<PlacedCard>>,
    mut resize_event: EventReader<WindowResized>,
) {
    for event in resize_event.read() {
        for (mut transform, card) in cards.iter_mut() {
            transform.translation.y = if card.player % 2 == 0 {
                (-event.height + CSH + 25.0) / 2.0
            } else {
                (-event.width + CSW + 25.0) / 2.0
            };
        }
        commands.run_system_cached(update_nametags);
    }
}

fn update_nametags(
    mut player_nametags: Query<(&mut Transform, &mut Text2d, &PlayerTag), Without<Card>>,
    game_info: Res<GameInfo>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    for (mut transform, mut text, name) in player_nametags.iter_mut() {
        let (i, player) = game_info
            .players
            .iter()
            .enumerate()
            .find(|(_, player)| player.name == name.0)
            .unwrap();
        text.0 = format!(
            "{}{}\n{}/{}\n{:.2}",
            name.0,
            if i == game_info.dealer { " (D)" } else { "" },
            player.taken,
            player.called,
            player.score as f32 / 100.0
        );
        transform.translation.y = if i % 2 == 0 {
            (-window.height() + CSH + text.lines().count() as f32 * HALF_FONT_HEIGHT) / 2.0 + CSH
        } else {
            (-window.width()
                + CSW
                + text.lines().map(|line| line.len()).max().unwrap() as f32 * HALF_FONT_HEIGHT)
                / 2.0
                + CSW
        }
    }
}
