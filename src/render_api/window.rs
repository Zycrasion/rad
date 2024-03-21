pub struct WindowOptions
{
    pub size : (u32, u32),
    pub title : String
}

impl Default for WindowOptions
{
    fn default() -> Self {
        Self { size: (480, 480), title: String::new() }
    }
}