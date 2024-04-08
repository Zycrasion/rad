use rad::*;

fn main()
{
    let mut app : App<OpenGL> = App::new();

    let mesh = Mesh{ handle:app.register_mesh(MeshBuilder::from_obj(include_str!("res/monkey.obj"))) };
    let transform = Transform::new();
    app.game.spawn((mesh, transform));
    app.game.add_systems(&ScheduleTimes::Update, rotate_monkey);

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