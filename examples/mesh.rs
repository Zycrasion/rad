use rad::*;

fn main()
{
    let mut app : App<OpenGL> = App::new();

    let mesh = Mesh{handle:app.register_mesh(MeshBuilder::from_obj(include_str!("res/monkey.obj")))};
    app.game.spawn(mesh);

    app.run();
    // App has taken complete control over the main thread, we are not getting control back
}

fn init_meshes()
{

}