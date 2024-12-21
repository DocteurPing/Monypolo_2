use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Toast {
    timer: Timer, // Timer to control the display duration
}

pub(crate) fn spawn_toast(commands: &mut Commands, message: String, duration: f32) {
    commands.spawn((
        Text(message),
        Toast {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        },
    ));
}

pub(crate) fn update_toasts(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Toast)>,
) {
    for (entity, mut toast) in query.iter_mut() {
        toast.timer.tick(time.delta());
        if toast.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
