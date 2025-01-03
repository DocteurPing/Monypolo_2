use crate::communication::MessageSender;
use crate::game_state::GamesState;
use bevy::color::Color;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use shared::action::{Action, PlayerAction};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

const BUY_BUTTON: &str = "Buy";
const SKIP_BUTTON: &str = "Skip";

#[derive(Component)]
pub(crate) struct BuyButton;

fn default_button(
    name: &str,
) -> (
    Button,
    Node,
    BorderColor,
    BorderRadius,
    BackgroundColor,
    Name,
    BuyButton,
) {
    (
        Button,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(NORMAL_BUTTON),
        Name::new(name.to_string()),
        BuyButton,
    )
}

pub(crate) fn spawn_buy_buttons(commands: &mut Commands, games_state: &mut GamesState) {
    games_state.buy_button_node_id = Some(
        commands
            .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(default_button(BUY_BUTTON))
                    .with_child((Text::new(BUY_BUTTON), TextColor(Color::srgb(0.9, 0.9, 0.9))));

                parent.spawn(default_button(SKIP_BUTTON)).with_child((
                    Text::new(SKIP_BUTTON),
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            })
            .id(),
    );
}

pub(crate) fn remove_buy_buttons(mut commands: Commands, games_state: Res<GamesState>) {
    if let Some(node_id) = games_state.buy_button_node_id {
        commands.entity(node_id).despawn_recursive();
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<BuyButton>),
    >,
    commands: Commands,
    games_state: Res<GamesState>,
    text_query: Query<&Text>,
    sender: Res<MessageSender>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let text = text_query.get(children[0]).unwrap();
        if *interaction == Interaction::Pressed {
            *color = PRESSED_BUTTON.into();
            border_color.0 = Color::WHITE;
            let sender = sender.clone();
            let task_pool = AsyncComputeTaskPool::get();
            let action = if **text == BUY_BUTTON {
                Action::BuyProperty
            } else {
                Action::SkipBuyProperty
            };
            task_pool
                .spawn(async move {
                    sender
                        .0
                        .send(PlayerAction {
                            action_type: action,
                            data: None,
                        })
                        .await
                        .unwrap();
                })
                .detach();
            remove_buy_buttons(commands, games_state);
            break;
        }
    }
}
