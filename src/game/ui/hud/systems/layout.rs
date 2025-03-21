use bevy::prelude::*;

use crate::game::{
    enemy::NUMBER_OF_ENEMIES,
    player::PLAYER_START_HEALTH,
    score::resources::Score,
    ui::hud::{components::*, styles::*},
};

pub fn spawn_game_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _hud_entity: Entity = build_hud(&mut commands, &asset_server);
}

fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: HUD_STYLE,
                ..default()
            },
            GameHUD {},
        ))
        // Info bar at the top-left of the screen
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: INFO_BAR_STYLE,
                    background_color: INFO_BAR_COLOR.into(),
                    ..default()
                })
                // Score info at the top
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: INFO_ITEM_STYLE,
                            ..default()
                        })
                        .with_children(|parent| {
                            let icon = asset_server.load("sprites/star.png");
                            parent.spawn(ImageBundle {
                                image: UiImage::new(icon),
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    ..default()
                                },
                                ..default()
                            });
                            parent.spawn((
                                TextBundle::from_section(
                                    format!("{:?}", Score::default().value),
                                    get_text_style(32.0, asset_server),
                                ),
                                ScoreInfo {},
                            ));
                        });
                })
                // Player health info in the center
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: INFO_ITEM_STYLE,
                            ..default()
                        })
                        .with_children(|parent| {
                            let icon = asset_server.load("sprites/info_heart.png");
                            parent.spawn(ImageBundle {
                                image: UiImage::new(icon),
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    ..default()
                                },
                                ..default()
                            });
                            parent.spawn((
                                TextBundle::from_section(
                                    format!("{:?}", PLAYER_START_HEALTH),
                                    get_text_style(32.0, asset_server),
                                ),
                                HealthInfo {},
                            ));
                        });
                })
                // Number of enemies info at the bottom
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: INFO_ITEM_STYLE,
                            ..default()
                        })
                        .with_children(|parent| {
                            let icon = asset_server.load("sprites/ball_red_large.png");
                            parent.spawn(ImageBundle {
                                image: UiImage::new(icon),
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    ..default()
                                },
                                ..default()
                            });
                            parent.spawn((
                                TextBundle::from_section(
                                    format!("{:?}", NUMBER_OF_ENEMIES),
                                    get_text_style(32.0, asset_server),
                                ),
                                EnemyNumberInfo {},
                            ));
                        });
                });
        })
        .id()
}
