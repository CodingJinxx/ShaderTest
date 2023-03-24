use bevy::{prelude::*, a11y::{AccessibilityNode, accesskit::{Role, NodeBuilder}}, input::mouse::{MouseWheel, MouseScrollUnit}, reflect::erased_serde::__private::serde::__private::de};

use crate::{loading::FontAssets, GameState, actions::Actions, actions::Tool};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
       app.add_system(setup_ui.in_schedule(OnEnter(GameState::Playing)))
       .add_system(update_debug_control_text.in_set(OnUpdate(GameState::Playing)))
       .add_system(handle_tool_buttons.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
pub enum ButtonType {
    Select,
    PlaceWall,
    PlaceLight,
    Delete
}

fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>) {

    commands.spawn(NodeBundle {
        style: Style {
            size: Size::width(Val::Percent(100.)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexEnd,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            ..default()
        }).with_children(|button_par| {
            button_par.spawn((ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },ButtonType::Select)).with_children(|buttons| {
                buttons.spawn(TextBundle::from_section("Select", TextStyle { font: font_assets.fira_sans.clone(), font_size: 12.0, color: Color::WHITE }));
            });

            button_par.spawn((ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },ButtonType::PlaceWall)).with_children(|buttons| {
                buttons.spawn(TextBundle::from_section("Place Wall", TextStyle { font: font_assets.fira_sans.clone(), font_size: 12.0, color: Color::WHITE }));
            });

            button_par.spawn((ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },ButtonType::PlaceLight)).with_children(|buttons| {
                buttons.spawn(TextBundle::from_section("Place Light", TextStyle { font: font_assets.fira_sans.clone(), font_size: 12.0, color: Color::WHITE }));
            });

            button_par.spawn((ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            }, ButtonType::Delete)).with_children(|buttons| {
                buttons.spawn(TextBundle::from_section("Delete", TextStyle { font: font_assets.fira_sans.clone(), font_size: 12.0, color: Color::WHITE }));
            });
        });

        parent.spawn((TextBundle::from_sections([
            TextSection::new("", TextStyle { font: font_assets.fira_sans.clone(), font_size: 20.0, color: Color::WHITE })
        ]), DebugControlsText));
    });
          
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

#[derive(Component, Default)]
struct DebugControlsText;

fn update_debug_control_text(mut debug_text: Query<&mut Text, With<DebugControlsText>>, actions: Res<Actions>) {
    let mut text = debug_text.single_mut();
    text.sections[0].value = format!("Clicked?: {:?},Current Tool: {:?}, Cursor X: {:.2}, Y: {:.2}, World Pos Cursor X: {:.2}, Y: {:.2}", actions.left_click,actions.current_tool().as_ref().unwrap_or(&Tool::None), actions.cursor_position_raw.unwrap_or(Vec2::ZERO).x, actions.cursor_position_raw.unwrap_or(Vec2::ZERO).y, actions.world_cursor_position.unwrap_or(Vec2::ZERO).x, actions.world_cursor_position.unwrap_or(Vec2::ZERO).y)
}

fn handle_tool_buttons(mut interaction_query: Query<(&Interaction, &ButtonType),(Changed<Interaction>, With<Button>)>, mut actions: ResMut<Actions>) {
    let mut just_set_ui_clicked = false;
    for interaction in interaction_query.iter() {
        match interaction.1 {
            ButtonType::Select => {
                if let Interaction::Clicked = interaction.0 {
                    actions.update_tool(Tool::Select);
                }
            },
            ButtonType::PlaceWall => {
                if let Interaction::Clicked = interaction.0 {
                    actions.update_tool(Tool::BuildWall);

                }
            },
            ButtonType::PlaceLight => {
                if let Interaction::Clicked = interaction.0 {
                    actions.update_tool(Tool::PlaceLight);
                }
            },
            ButtonType::Delete => {
                if let Interaction::Clicked = interaction.0 {
                    actions.update_tool(Tool::Delete);
                }
            }
        }
        actions.ui_just_clicked = true;
        just_set_ui_clicked = true;
    }
    if(just_set_ui_clicked == false) {
        actions.ui_just_clicked = false;
    }
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size().y)
                .sum();
            let panel_height = uinode.size().y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}