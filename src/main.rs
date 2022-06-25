use bevy::{
    core::FixedTimestep,
    math::{const_vec2, Vec3Swizzles},
    prelude::*,
};

const TIME_STEP: f32 = 1.0 / 60.0;
const BOUNDS: Vec2 = const_vec2!([1200.0, 640.0]);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GameState>()
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement_system)
                .with_system(player_shooting_system),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run()
}

#[derive(Component, Debug)]
struct Player {
    /// linear speed in meters per second
    velocity: f32,
    /// rotation speed in radians per second
    rotation_speed: f32,
}

#[derive(Component, Debug, Default)]
struct Bullet {
    velocity: f32,
    direction: Vec3,
}

#[derive(Default)]
struct GameState {
    score: usize,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
) {
    game_state.score = 0;

    let player = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(25.0, 25.0)),
            ..default()
        },
        ..default()
    };

    let bullet = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..default()
        },
        ..default()
    };

    // let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let text_style = TextStyle {
        font_size: 60.0,
        color: Color::BLUE,
        ..Default::default()
    };

    let text_alignment_topleft = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Left,
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            game_state.score.to_string(),
            text_style,
            text_alignment_topleft,
        ),
        ..Default::default()
    });
    commands.spawn_bundle(bullet).insert(Bullet {
        velocity: 1000.0,
        direction: 1.0 * Vec3::Y,
    });
    commands.spawn_bundle(player).insert(Player {
        velocity: 500.0,
        rotation_speed: f32::to_radians(360.0),
    });
}

fn player_shooting_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    bullet_entities: Query<Entity, With<Bullet>>,
    mut set: ParamSet<(
        Query<(&Bullet, &mut Transform)>,
        Query<(&Player, &Transform)>,
    )>,
) {
    let player_query = set.p1();
    let (_, player_transform) = player_query.single();

    let player_position = player_transform.translation;
    let player_direction = player_transform.rotation * Vec3::Y;

    let bullet = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..default()
        },
        transform: Transform::from_translation(player_position),
        ..default()
    };
    let new_bullet = Bullet {
        velocity: 1000.0,
        direction: player_direction,
    };
    commands.spawn_bundle(bullet).insert(new_bullet);

    for entity in bullet_entities.iter() {
        if let Ok((bullet, mut bullet_transform)) = set.p0().get_mut(entity) {
            let distance = bullet.velocity * TIME_STEP;
            let movement_delta = distance * bullet.direction;
            bullet_transform.translation += movement_delta;

            let extents = Vec3::from((BOUNDS / 2.0, 0.0));
            if bullet_transform.translation.gt(&extents)
                || bullet_transform.translation.lt(&-extents)
            {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut velocity = 0.0 * Vec3::X;

    if keyboard_input.pressed(KeyCode::Q) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::E) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        velocity += 1.0 * Vec3::Y;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        velocity -= 1.0 * Vec3::Y;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        velocity -= 1.0 * Vec3::X;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        velocity += 1.0 * Vec3::X;
    }

    let rotation_delta = Quat::from_rotation_z(rotation_factor * player.rotation_speed * TIME_STEP);
    transform.rotation *= rotation_delta;

    let movement_distance = player.velocity * TIME_STEP;
    let translation_delta = velocity * movement_distance;
    transform.translation += translation_delta;

    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}
