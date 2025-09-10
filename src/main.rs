use bevy::{platform::collections::HashMap, prelude::*};

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

fn setup(mut commands: Commands, mut assets: ResMut<CardAssets>, asset_server: Res<AssetServer>) {
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

    let mut players = HashMap::new();
    for name in ["lela", "ilia", "lizi", "giorgi"] {
        players.insert(name, Vec::new());
    }
    for cards in players.values_mut() {
        for _ in 0..9 {
            let card = deck.remove(rand::random_range(0..deck.len()));
            cards.push(card);
        }
    }
}
