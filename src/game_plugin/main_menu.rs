use bevy::prelude::*;
use crate::{game_assets::FontAssets, GameState};

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct MainMenuCamera;

#[derive(Component)]
pub struct MainMenuPlayButton;

pub struct MainMenuPlugin;
impl MainMenuPlugin {
    fn spawn_main_menu_camera(mut commands: Commands) {
        commands
            .spawn_bundle(UiCameraBundle::default())
            .insert(MainMenuCamera);
    }

    fn spawn_main_menu_ui(mut commands: Commands, fonts: Res<FontAssets>) {
        // Main Menu Root
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::rgb_u8(60, 60, 60).into(),
                ..default()
            })
            .insert(MainMenu)
            .with_children(|parent| {
                // Title Text
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Character Creator",
                        TextStyle {
                            font: fonts.title.clone(),
                            color: Color::rgb_u8(220, 220, 220),
                            font_size: 45.0,
                        },
                        TextAlignment::default(),
                    ),
                    ..default()
                });

                // Sub Title Text
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Unintended Consequences",
                        TextStyle {
                            font: fonts.general.clone(),
                            color: Color::rgb_u8(180, 180, 180),
                            font_size: 26.0,
                        },
                        TextAlignment::default(),
                    ),
                    style: Style {
                        margin: Rect {
                            bottom: Val::Px(30.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                });

                // Play Button
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(200.0), Val::Px(30.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        color: Color::rgb_u8(42, 42, 42).into(),
                        ..default()
                    })
                    .insert(MainMenuPlayButton)
                    .with_children(|parent| {
                        // Button Child Text
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Create",
                                TextStyle {
                                    font: fonts.general.clone(),
                                    color: Color::rgb_u8(216, 216, 216),
                                    font_size: 24.0,
                                },
                                TextAlignment::default(),
                            ),
                            ..default()
                        });
                    });
            });
    }

    fn handle_play_button(
        mut state: ResMut<State<GameState>>,
        buttons: Query<&Interaction, With<MainMenuPlayButton>>,
    ) {
        for button in buttons.iter() {
            if *button == Interaction::Clicked {
                state.set(GameState::CharSelect).unwrap();
                return;
            }
        }
    }

    fn interaction_styles(mut buttons: Query<(&Interaction, &mut UiColor), With<Button>>) {
        for (interaction, mut color) in buttons.iter_mut() {
            color.0 = match interaction {
                Interaction::Hovered => Color::rgb_u8(107, 107, 107),
                _ => Color::rgb_u8(42, 42, 42),
            };
        }
    }

    fn despawn_main_menu_ui(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn despawn_main_menu_camera(
        mut commands: Commands,
        query: Query<Entity, With<MainMenuCamera>>,
    ) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(Self::spawn_main_menu_camera)
                .with_system(Self::spawn_main_menu_ui),
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(Self::handle_play_button)
                .with_system(Self::interaction_styles),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::MainMenu)
                .with_system(Self::despawn_main_menu_ui)
                .with_system(Self::despawn_main_menu_camera),
        );
    }
}
