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
pub struct RotatingMesh(pub f32);

fn main()
{
    let mut app : App<OpenGL> = App::new();

    let mesh = Mesh{ handle:app.register_mesh(MeshBuilder::from_obj(include_str!("res/monkey.obj"))) };
    
    // Monkeys
    app.game.spawn((mesh.clone(), Transform::with_position(0., -2., 5.5), RotatingMesh(0.05)));
    app.game.spawn((mesh.clone(), Transform::with_position(0., 0., 5.5), RotatingMesh(0.1)));
    app.game.spawn((mesh.clone(), Transform::with_position(0., 2., 5.5), RotatingMesh(0.2)));
    
    // Camera
    app.game.spawn(CameraBundle::new());

    // To Track Elapsed Time from Start
    app.game.world.insert_resource(Time(Instant::now()));

    // Update Mesh and Camera Positions
    app.game.add_systems(&ScheduleTimes::Update, (rotate_monkey, move_camera));

    app.run();

    println!("Exiting App");
}

fn rotate_monkey(mut query : Query<(&Mesh, &RotatingMesh, &mut Transform)>)
{
    for (_, rotating_mesh, mut transform) in query.iter_mut()
    {
        transform.rotation.y += rotating_mesh.0;
    }
}

fn move_camera(mut query : Query<(&Camera, &mut Transform)>, time : Res<Time>)
{
    for (_, mut transform) in query.iter_mut()
    {
        transform.position.z = time.elapsed().sin() + 2.5;
        transform.position.x = time.elapsed().cos();
    }
}