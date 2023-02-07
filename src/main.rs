use bevy::{prelude::*, input::mouse::MouseMotion};

const CLEAR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1000.,
                height: 1000.,
                title: "rust game".to_owned(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                resizable: true,
                ..default()
            },
            
            ..default()
        }))
        .add_plugin(StartUp)
        .add_system(move_towards_cursor)
        .add_system(move_particle)
       //  .add_system(window_resize_system)
        .run();
}

#[derive(Component)]
struct Particle {
    angle: f32,
}
fn spawn_view(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_particle(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::Rgba {
                    red: 0.5,
                    green: 0.5,
                    blue: 0.5,
                    alpha: 0.5,
                },
                custom_size: Some(Vec2 { x: 10.0, y: 10.0 }),
                ..default()
            },
            transform: Transform {
                rotation: Quat::from_rotation_z(45.0),
                translation: Vec3::new(10.0, -30.0, 0.0),
                ..default()
            },
            ..default()
        },
        Particle { angle: 45.0 },
    ));
}

fn move_particle(time: Res<Time>, mut query: Query<(&mut Particle, &mut Transform)>) {
    let (mut particle, mut transform) = query.single_mut();
    let quat = Quat::from_rotation_z(particle.angle);
    transform.translation += quat * Vec3::new(10., 0., 0.);
}



fn move_towards_cursor(windows: Res<Windows>, mut query: Query<(&mut Particle, &Transform)>) {
    let window = windows.get_primary().unwrap();
    let (mut particle, transform) = query.single_mut();
    if let Some(position) = window.cursor_position() {
        particle.angle = find_angle(transform.translation.truncate(), position + Vec2::new(-500., -500.));

    }
}

fn find_angle(observer: Vec2, target: Vec2) -> f32 {
    let result: Vec2 = target - observer;
    result.y.atan2(result.x)
}

struct StartUp;

impl Plugin for StartUp {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_view)
            .add_startup_system(spawn_particle);
    }
}

fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    println!("Logical size was: {},{}", window.width(), window.height());
    println!("Physical size was: {},{}", window.physical_width(), window.physical_height());
    println!("Scale Factor was: {}", window.scale_factor());
}
