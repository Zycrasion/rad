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

fn main()
{
    let mut app : App<OpenGL> = App::new();

    let mesh = Mesh{ handle:app.register_mesh(MeshBuilder::from_obj(include_str!("res/monkey.obj"))) };
    let mut monkey_transform = Transform::new();
    monkey_transform.position = Vector::new3(0., 0., 5.5);
    app.game.spawn((mesh, monkey_transform));
    app.game.spawn(CameraBundle::new());
    app.game.world.insert_resource(Time(Instant::now()));
    app.game.add_systems(&ScheduleTimes::Update, (rotate_monkey, move_camera));

    app.run();
    // App has taken complete control over the main thread, we are not getting control back
}

fn rotate_monkey(mut query : Query<(&Mesh, &mut Transform)>)
{
    for (_, mut transform) in query.iter_mut()
    {
        transform.rotation.y += 0.1;
    }
}

fn move_camera(mut query : Query<(&Camera, &mut Transform)>, time : Res<Time>)
{
    for (_, mut transform) in query.iter_mut()
    {
        transform.position.z = time.elapsed().sin() * 2.5 + 2.5;
    }
}