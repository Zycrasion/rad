use std::{collections::HashMap, fs::File, hash::Hash, io::Write, thread, time::Duration};

use glfw::{fail_on_errors, Context, Glfw, GlfwReceiver, WindowEvent};

use crate::{ControlFlow, GameManager, InputAction, Key, RenderAPI};

const API_NAME : &str = "OpenGL4";

pub struct OpenGL4
{
    window : glfw::PWindow,
    events : GlfwReceiver<(f64, WindowEvent)>,
    glfw : Glfw,
}

impl OpenGL4
{

    fn _resize(&mut self, size : (i32, i32))
    {
        let (width, height) = if size.0 <= 0 || size.1 <= 0
        {
            self.log_error("Size component was non-zero");
            
            // Try to get current window size
            let size = self.window.get_size();
            if size.0 <= 0 || size.1 <= 0 {self.log_error("FATAL ERROR: Couldn't Get Window Size"); panic!()}
            (size.0.unsigned_abs(), size.1.unsigned_abs())
        } else {(size.0.unsigned_abs(), size.1.unsigned_abs())};

        // Resize code
        self.log_debug(&format!("Resize Event: {} {}", width, height));
    }

    fn _collect_events(&self) -> Vec<(f64, WindowEvent)>
    {
        glfw::flush_messages(&self.events).collect()
    }
}

impl RenderAPI for OpenGL4
{
    fn init_with_window(options : crate::WindowOptions) -> Self {
        let mut glfw = glfw::init_no_callbacks().unwrap();
        let window = glfw.create_window(options.size.0, options.size.1, &options.title, glfw::WindowMode::Windowed).expect("Unable to create a window");
        OpenGL4
        {
            window: window.0,
            events: window.1,
            glfw,
        }
    }
    
    fn take_control(mut self, mut manager : GameManager) -> ! {
        self.window.make_current();
        self.window.set_all_polling(true);
        let mut control_flow = ControlFlow::Continue;

        while !self.window.should_close()
        {
            self.window.swap_buffers();
            self.glfw.poll_events();

            if control_flow == ControlFlow::Exit
            {
                self.window.set_should_close(true);
            }

            let events =  self._collect_events();
            for (_, event) in events
            {
                match event
                {
                    WindowEvent::Size(width, height) => self._resize((width, height)),
                    WindowEvent::Key(key, _, action, _) => 
                    {  
                        // Dont override control_flow if its set to Exit and GameManager returns ControlFlow::Continue
                        if manager.key_press(Key::from_glfw_key(key), InputAction::from_glfw_action(action)) == ControlFlow::Exit
                        {
                            control_flow = ControlFlow::Exit;
                        }
                    },
                    _ => {}
                }
            }
            thread::sleep(Duration::from_secs_f64(1. / 60.));
        }

        panic!()
    }
    
    fn log_error(&self, message : &str) {
        eprintln!("(ERROR)[{}]: {}", API_NAME, message)
    }
    
    #[cfg(not(debug_assertions))]
    fn log_debug(&self, message : &str)
    {
    }

    #[cfg(debug_assertions)]
    fn log_debug(&self, message : &str) {
        println!("(DEBUG)[{}]: {}", API_NAME, message)
    }
}