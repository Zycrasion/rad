use rad::*;

fn main()
{
    let mut app : App<OpenGL> = App::new();

    app.game.add_systems(&Startup, init_meshes);

    app.run();
    // App has taken complete control over the main thread, we are not getting control back
}

fn init_meshes()
{

}