use crate::communication::MessageSender;
use crate::game_state::GamesState;
use crate::screens::GameStateEnum;
use crate::tools::despawn_screen;
use crate::ui::toast::{spawn_toast, ToastCount};
use bevy::color::palettes::css::CRIMSON;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::{app::AppExit, prelude::*};
use bevy_simple_text_input::{
    TextInput, TextInputPlugin, TextInputSubmitEvent, TextInputSystem, TextInputTextColor,
    TextInputTextFont,
};
use shared::action::{Action, PlayerAction};

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.52, 0.99);
const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

// This plugin manages the menu, with 5 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub fn menu_plugin(app: &mut App) {
    app
        // At start, the menu is not enabled. This will be changed in `menu_setup` when
        // entering the `GameState::Menu` state.
        // Current screen in the menu is handled by an independent state from `GameState`
        .init_state::<MenuState>()
        .add_plugins(TextInputPlugin)
        .add_systems(OnEnter(GameStateEnum::Menu), menu_setup)
        // Systems to handle the main menu screen
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(Update, listener.after(TextInputSystem))
        // Common systems to all screens that handles buttons behavior
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameStateEnum::Menu)),
        );
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    #[default]
    Disabled,
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

// This system handles changing all buttons color based on mouse interaction
#[allow(clippy::type_complexity)]
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn((
                        Text::new("Bevy Game Menu UI"),
                        TextFont {
                            font_size: 67.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ));
                    parent
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Node {
                                    width: Val::Px(200.0),
                                    border: UiRect::all(Val::Px(5.0)),
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                BorderColor(BORDER_COLOR_ACTIVE),
                                BackgroundColor(BACKGROUND_COLOR),
                                TextInput,
                                TextInputTextFont(TextFont {
                                    font_size: 34.,
                                    ..default()
                                }),
                                TextInputTextColor(TextColor(TEXT_COLOR)),
                            ));
                        });

                    // Display three buttons for each action available from the main menu:
                    // - new game
                    // - settings
                    // - quit
                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("sprites/right.png");
                            parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                            parent.spawn((
                                Text::new("New Game"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });
                    parent
                        .spawn((
                            Button,
                            button_node,
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("sprites/exitRight.png");
                            parent.spawn((ImageNode::new(icon), button_icon_node));
                            parent.spawn((
                                Text::new("Quit"),
                                button_text_font,
                                TextColor(TEXT_COLOR),
                            ));
                        });
                });
        });
}

#[allow(clippy::type_complexity)]
fn menu_action(
    mut commands: Commands,
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state_enum: ResMut<NextState<GameStateEnum>>,
    state: ResMut<GamesState>,
    toast_count: ResMut<ToastCount>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    if state.id.is_nil() {
                        spawn_toast(
                            &mut commands,
                            "Please enter a name".to_string(),
                            3.0,
                            toast_count,
                        );
                        break;
                    } else {
                        game_state_enum.set(GameStateEnum::Game);
                        menu_state.set(MenuState::Disabled);
                    }
                }
            }
        }
    }
}

fn listener(sender: Res<MessageSender>, mut events: EventReader<TextInputSubmitEvent>) {
    for event in events.read() {
        let value = event.value.clone();
        if value.is_empty() {
            return;
        }
        let sender = sender.clone();
        let task_pool = AsyncComputeTaskPool::get();
        task_pool
            .spawn(async move {
                sender
                    .0
                    .send(PlayerAction {
                        action_type: Action::Identify,
                        data: Some(value),
                    })
                    .await
                    .unwrap();
            })
            .detach();
        info!("{:?} submitted: {}", event.entity, event.value);
    }
}
