//! A shader that uses dynamic data like the time since startup.
//! The time data is in the globals binding which is part of the `mesh_view_bindings` shader import.

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(Component)]
struct MyCamera;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(CustomMaterial {}),
        ..default()
    });
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        transform: Transform::from_xyz(3.0, 0.5, 0.0),
        material: materials.add(CustomMaterial {}),
        ..default()
    });
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        transform: Transform::from_xyz(-3.0, 0.5, 0.0),
        material: materials.add(CustomMaterial {}),
        ..default()
    });
    
    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
        MyCamera,
    ));
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut q: Query<&mut Transform, With<MyCamera>>,
    time: Res<Time>,
){
    if let Ok(mut camera_transform) = q.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut angle = 0.0;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::Z) {
            direction += Vec3::new(0.0, 0.0, 1.0)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::Q) {
            direction += Vec3::new(0.0, 0.0, -1.0)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::BracketLeft) {
            angle = -0.001;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::BracketRight) {
            angle = 0.001;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            camera_transform.translation += direction * time.delta_seconds();
        }

        if angle != 0.0 {
            camera_transform.rotate_y(angle)
        }

    }
}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animate_shader.wgsl".into()
    }
}
