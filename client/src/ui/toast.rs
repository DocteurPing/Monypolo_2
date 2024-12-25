use bevy::prelude::*;
use std::fmt::Debug;

const TOAST_OFFSET: f32 = 30.0;

#[derive(Component)]
pub(crate) struct Toast {
    timer: Timer, // Timer to control the display duration
}

#[derive(Resource, Debug)]
pub(crate) struct ToastCount(pub usize);

pub(crate) fn spawn_toast(
    commands: &mut Commands,
    message: String,
    duration: f32,
    mut toast_count: ResMut<ToastCount>,
) {
    commands.spawn((
        Text(message),
        Node {
            top: Val::Px(TOAST_OFFSET * toast_count.0 as f32),
            ..default()
        },
        Toast {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        },
    ));
    toast_count.0 += 1;
}

pub(crate) fn update_toasts(
    mut commands: Commands,
    time: Res<Time>,
    mut list_toast: Query<(Entity, &mut Toast), With<Toast>>,
    mut nodes: Query<&mut Node, With<Toast>>,
    mut toast_count: ResMut<ToastCount>,
) {
    for (entity, mut toast) in list_toast.iter_mut() {
        toast.timer.tick(time.delta());
        if toast.timer.finished() {
            commands.entity(entity).despawn();
            toast_count.0 -= 1;
            for mut node in nodes.iter_mut() {
                // Move up the node position
                if let Val::Px(top) = node.top {
                    node.top = Val::Px(top - TOAST_OFFSET);
                }
                //node.top = Val::Px(node.top .unwrap().get() - TOAST_OFFSET);
            }
        }
    }

    // for mut transform in list_transform.iter_mut() {
    //     transform.translation.y -= TOAST_OFFSET;
    // }
    // if we despawn a toast, we need to update the position of the remaining toasts
    // for (entity, _, mut transform) in list_toast.iter_mut() {
    //     transform.translation.y -= TOAST_OFFSET * 100.0;
    // }
}
