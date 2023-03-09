use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rewind::*;

#[rustfmt::skip]
fn main() {
    println!();
    println!("Hold the Space key to rewind");
    println!();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())

        // Setup the scene
        .add_system(setup.on_startup())

        // Enable rewind when holding the space key
        .add_system(enable_rewinding)

        // Enable rigidbodies when bot rewinding
        .add_system(enable_rigidbodies.run_if(not(rewinding)))

        // Disable rigidbodies when rewinding so rapier doesn't try to simulate physics when rewinding
        .add_system(disable_rigidbodies.run_if(rewinding))

        // Add the rewind plugin
        .add_plugin(RewindPlugin {
            cancel_rewind_on_empty_history: true,
            ..Default::default()
        })

        // Add rewind surport for Velocity component
        .init_rewind_component::<Velocity>()

        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(20.0, 20.0, 20.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    // Spawn Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::default().looking_at(Vec3::new(1.0, -3.0, 1.0), Vec3::Y),
        ..Default::default()
    });

    // Spawn Ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(40.0, 0.5, 40.0).into()),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..Default::default()
        },
        Collider::cuboid(20.0, 0.5, 20.0),
    ));

    // Spawn Spheres

    let mesh_handle = meshes.add(shape::UVSphere::default().into());
    let material_handle = materials.add(Color::RED.into());

    for n in 0..10 {
        commands.spawn((
            PbrBundle {
                mesh: mesh_handle.clone(),
                material: material_handle.clone(),
                transform: Transform::from_xyz(n as f32 * 0.1, n as f32 * 2.5, -n as f32 * 0.1),
                ..Default::default()
            },
            Collider::ball(1.0),
            RigidBody::Dynamic,
            Velocity::default(),
            RewindComponent::<Transform>::default(), // Track the Transform component
            RewindComponent::<Velocity>::default(),  // Track the Velocity component
        ));
    }
}

// Systems needed for the demo

fn enable_rewinding(mut rewind: ResMut<Rewind>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        rewind.rewinding = true;
    } else if keys.just_released(KeyCode::Space) {
        rewind.rewinding = false;
    }
}

fn enable_rigidbodies(mut query: Query<&mut RigidBody>) {
    for mut rb in &mut query {
        *rb = RigidBody::Dynamic;
    }
}

fn disable_rigidbodies(mut query: Query<&mut RigidBody>) {
    for mut rb in &mut query {
        *rb = RigidBody::Fixed;
    }
}
