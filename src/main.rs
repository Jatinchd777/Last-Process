use bevy::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::new(40.0, 64.0);
const MOVE_SPEED: f32 = 300.0;
const JUMP_SPEED: f32 = 650.0;
const GRAVITY: f32 = 1800.0;

#[derive(Component)]
struct Player {
    velocity: Vec2,
    grounded: bool,
}

#[derive(Component)]
struct Platform {
    size: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Player
    commands.spawn((
        Sprite::from_color(Color::srgb(0.025, 0.025, 0.035), PLAYER_SIZE),
        Transform::from_xyz(0.0, -100.0, 2.0),
        Player {
            velocity: Vec2::ZERO,
            grounded: false,
        },
    ));

    // Main floor
    spawn_platform(
        &mut commands,
        Vec2::new(0.0, -180.0),
        Vec2::new(1200.0, 48.0),
    );

    // Platforms to jump onto
    spawn_platform(
        &mut commands,
        Vec2::new(260.0, -90.0),
        Vec2::new(180.0, 24.0),
    );

    spawn_platform(
        &mut commands,
        Vec2::new(500.0, 10.0),
        Vec2::new(160.0, 24.0),
    );
}

fn spawn_platform(commands: &mut Commands, position: Vec2, size: Vec2) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.16, 0.16, 0.19), size),
        Transform::from_xyz(position.x, position.y, 0.0),
        Platform { size },
    ));
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    platforms: Query<(&Transform, &Platform), Without<Player>>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut player) in &mut player_query {
        let was_grounded = player.grounded;

        // Horizontal movement
        let mut direction = 0.0;

        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }

        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }

        let old_x = transform.translation.x;
        let half_width = PLAYER_SIZE.x / 2.0;
        let half_height = PLAYER_SIZE.y / 2.0;

        transform.translation.x += direction * MOVE_SPEED * dt;

        // Stop the player from walking through platform sides.
        for (platform_transform, platform) in &platforms {
            let px = platform_transform.translation.x;
            let py = platform_transform.translation.y;

            let left = px - platform.size.x / 2.0;
            let right = px + platform.size.x / 2.0;
            let bottom = py - platform.size.y / 2.0;
            let top = py + platform.size.y / 2.0;

            let player_bottom = transform.translation.y - half_height;
            let player_top = transform.translation.y + half_height;

            let overlaps_y = player_bottom < top && player_top > bottom;

            let old_right = old_x + half_width;
            let old_left = old_x - half_width;
            let new_right = transform.translation.x + half_width;
            let new_left = transform.translation.x - half_width;

            if overlaps_y && direction > 0.0 && old_right <= left && new_right > left {
                transform.translation.x = left - half_width;
            } else if overlaps_y && direction < 0.0 && old_left >= right && new_left < right {
                transform.translation.x = right + half_width;
            }
        }

        // Jump only when grounded.
        if was_grounded
            && (keys.just_pressed(KeyCode::Space)
                || keys.just_pressed(KeyCode::KeyW)
                || keys.just_pressed(KeyCode::ArrowUp))
        {
            player.velocity.y = JUMP_SPEED;
        }

        // Gravity
        player.velocity.y -= GRAVITY * dt;

        let old_bottom = transform.translation.y - half_height;

        transform.translation.y += player.velocity.y * dt;

        let new_bottom = transform.translation.y - half_height;

        player.grounded = false;

        // Land on platforms while falling.
        if player.velocity.y <= 0.0 {
            for (platform_transform, platform) in &platforms {
                let px = platform_transform.translation.x;
                let py = platform_transform.translation.y;

                let left = px - platform.size.x / 2.0;
                let right = px + platform.size.x / 2.0;
                let top = py + platform.size.y / 2.0;

                let player_left = transform.translation.x - half_width;
                let player_right = transform.translation.x + half_width;

                let overlaps_x = player_right > left && player_left < right;

                let crossed_top = old_bottom >= top && new_bottom <= top;

                if overlaps_x && crossed_top {
                    transform.translation.y = top + half_height;
                    player.velocity.y = 0.0;
                    player.grounded = true;
                    break;
                }
            }
        }
    }
}
