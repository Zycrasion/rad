use rad::*;

fn main()
{
    let app : App = App::new();
 
    // App has taken complete control over the main thread, we are not getting control back
    app.run();
}
