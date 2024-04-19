use std::time::Instant;
use rad::*;

#[derive(Resource)]
pub struct Time(pub Instant);

impl Time
{
    pub fn elapsed(&self) -> f32
    {
        self.0.elapsed().as_secs_f32()
    }
}

#[derive(Component)]
pub struct Rotate(pub f32);

#[derive(Component)]
pub struct MoveInCircle(Vector, f32, f32);

#[derive(Component)]
pub struct RainbowLight;

fn main()
{
    let mut app = App::new();

    let mesh = app.register_mesh(MeshBuilder::from_obj(include_str!("res/monkey.obj")));
    
    // Monkeys
    app.spawn((DefaultMaterial::new(Colour(0.1, 0.2, 0.3)), mesh.clone(), Transform::with_position(0., -2., 5.5), Rotate(0.05)));
    
    for x in 0..=100
    {
        app.spawn((DefaultMaterial::new(Colour(x as f32/100., 0., 0.3)), mesh.clone(), Transform::with_position(x as f32, 0., 5.5 + x as f32), Rotate(0.1)));
    }

    app.spawn((DefaultMaterial::new(Colour(0.1, 0.2, 0.3)), mesh.clone(), Transform::with_position(0., 2., 5.5), Rotate(0.2)));

    app.spawn((DefaultMaterial {shading_enabled : false, ..Default::default()}, Light {colour : Colour::WHITE}, mesh.clone(), Transform::with_position(-3., 2., 5.5)));
    
    // Camera
    let mut cam_bundle = CameraBundle::new();
    cam_bundle.camera.draw_params.clear_colour = Some((0., 0., 0.2, 1.));
    app.spawn((MoveInCircle(Vector::new2(0., 0.), 2.5, 0.), cam_bundle));

    // To Track Elapsed Time from Start
    app.world.insert_resource(Time(Instant::now()));

    // Update Mesh and Camera Positions
    app.add_systems(ScheduleTimes::Update, (rotating, move_in_circle));

    app.run();
}

fn rotating(mut query : Query<(&Rotate, &mut Transform)>)
{
    for (rotating_mesh, mut transform) in query.iter_mut()
    {
        transform.rotation.y += rotating_mesh.0;
    }
}

fn move_in_circle(mut query : Query<(&MoveInCircle, &mut Transform)>, time : Res<Time>)
{
    for (circle_params, mut transform) in query.iter_mut()
    {
        transform.position.z = (time.elapsed() + circle_params.2).sin() * circle_params.1 + circle_params.0.z;
        transform.position.x = (time.elapsed() + circle_params.2).cos() * circle_params.1 + circle_params.0.x;
        transform.position.y = circle_params.0.y;
    }
}