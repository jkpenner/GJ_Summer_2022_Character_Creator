use bevy::prelude::*;

use crate::{game_assets::FontAssets, GameState};

#[derive(Component)]
pub struct CharSelect;

#[derive(Component)]
pub struct CharSelectCamera;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct CreateButton;

pub struct CharSelectPlugin;

impl CharSelectPlugin {
    fn spawn_char_select_camera(mut commands: Commands) {
        commands
            .spawn_bundle(UiCameraBundle::default())
            .insert(CharSelectCamera);
    }

    fn spawn_char_select_ui(mut commands: Commands, fonts: Res<FontAssets>) {
        let exit_button =
                    Self::button(&mut commands, "Exit", fonts.button.clone(), ExitButton);
                let create_button =
                    Self::button(&mut commands, "Create", fonts.button.clone(), CreateButton);

        // Main Menu Root
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..default()
                },
                color: Color::rgb_u8(60, 60, 60).into(),
                ..default()
            })
            .insert(CharSelect)
            .with_children(|parent| {
                // Header Section
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
                            padding: Rect::all(Val::Px(10.0)),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::FlexEnd,
                            ..default()
                        },
                        color: Color::RED.into(),
                        ..default()
                    })
                    .with_children(|header| {
                        header.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Create Character",
                                TextStyle {
                                    font: fonts.title.clone(),
                                    color: Color::rgb_u8(220, 220, 220),
                                    font_size: 30.0,
                                },
                                TextAlignment::default(),
                            ),
                            ..default()
                        });
                    });

                // Content Section
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        flex_grow: 1.0,
                        flex_shrink: 1.0,
                        ..default()
                    },
                    color: Color::BLUE.into(),
                    ..default()
                });

                

                // Footer Section
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
                            padding: Rect::all(Val::Px(10.0)),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::FlexStart,
                            ..default()
                        },
                        color: Color::YELLOW.into(),
                        ..default()
                    })
                    .add_child(exit_button)
                    .add_child(create_button);
            });
    }

    fn button(
        cmds: &mut Commands,
        text: &str,
        font: Handle<Font>,
        marker: impl Component,
    ) -> Entity {
        cmds.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(30.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb_u8(42, 42, 42).into(),
            ..default()
        })
        .insert(CreateButton)
        .with_children(|parent| {
            // Button Child Text
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font: font,
                        color: Color::rgb_u8(216, 216, 216),
                        font_size: 24.0,
                    },
                    TextAlignment::default(),
                ),
                ..default()
            });
        })
        .id()
    }

    fn handle_create_button(
        mut state: ResMut<State<GameState>>,
        buttons: Query<&Interaction, With<CreateButton>>,
    ) {
        for button in buttons.iter() {
            if *button == Interaction::Clicked {
                state.set(GameState::GamePlay).unwrap();
                return;
            }
        }
    }

    fn handle_exit_button(
        mut state: ResMut<State<GameState>>,
        buttons: Query<&Interaction, With<ExitButton>>,
    ) {
        for button in buttons.iter() {
            if *button == Interaction::Clicked {
                state.set(GameState::MainMenu).unwrap();
                return;
            }
        }
    }

    fn despawn_char_select_ui(mut commands: Commands, query: Query<Entity, With<CharSelect>>) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn despawn_char_select_camera(
        mut commands: Commands,
        query: Query<Entity, With<CharSelectCamera>>,
    ) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

impl Plugin for CharSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::CharSelect)
                .with_system(Self::spawn_char_select_camera)
                .with_system(Self::spawn_char_select_ui),
        )
        .add_system_set(
            SystemSet::on_update(GameState::CharSelect)
                .with_system(Self::handle_create_button)
                .with_system(Self::handle_exit_button),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::CharSelect)
                .with_system(Self::despawn_char_select_camera)
                .with_system(Self::despawn_char_select_ui),
        );
    }
}
