use rad::*;

fn main()
{
    let app : App<OpenGL4> = App::new();
    app.run();
    // App has taken complete control over the main thread, we are not getting control back
}