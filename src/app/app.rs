use crate::*;

pub struct GameManager
{}

impl GameManager {
    pub fn key_press(&mut self, key : Key, input_action : InputAction) -> ControlFlow
    {
        if key == Key::Escape
        {
            return ControlFlow::Exit
        }

        ControlFlow::Continue
    }

    pub fn mouse_move(&mut self, x : f64, y : f64)
    {

    }

    pub fn mouse_button_down(&mut self, button : MouseButton, action : InputAction)
    {

    }
}

pub struct App<T : RenderAPI>
{
    backend : T,
    game_manager : GameManager
}

impl<T : RenderAPI> App<T>
{
    pub fn new() -> Self
    {
        App
        {
            backend : T::init_with_window(WindowOptions
                {
                    size : (480, 480),
                    title :  String::from("RenderAPI Test")
                }),
            game_manager : GameManager{},
        }
    }

    pub fn run(mut self)
    {
        self.backend.take_control(GameManager{});
    }
}