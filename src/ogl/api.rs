use std::{collections::HashMap, fs::File, hash::Hash, io::Write, thread, time::{Duration, Instant}};

use glium::{backend::glutin::{self, SimpleWindowBuilder}, glutin::{config::{self, ConfigTemplateBuilder}, surface::WindowSurface}, Display, Surface};
use winit::{dpi::{PhysicalSize, Size}, event::{Event, WindowEvent}, event_loop::{self, EventLoop, EventLoopBuilder, EventLoopWindowTarget}, window::{Window, WindowBuilder, WindowId}};
use glium::glutin::prelude::*;
use crate::{ControlFlow, GameManager, InputAction, Key, RenderAPI};

const API_NAME : &str = "OpenGL4";

pub struct OpenGL
{
    event_loop : Option<EventLoop<()>>,
    window : Window,
    display : Display<WindowSurface>,
    last_frame : Instant,
    target_frame_rate : f64
}

impl OpenGL
{
    fn _event_loop(&mut self, event : Event<()>, target : &EventLoopWindowTarget<()> ,manager : &mut GameManager)
    {
        match event
        {
            Event::WindowEvent { window_id, event } => self._window_event(target, window_id, event, manager),
            Event::AboutToWait => 
            {
                if self.delta_time() > 1. / self.target_frame_rate
                {
                    self.window.request_redraw();
                }
            },``
            _ => {}
        }
    }

    /// Returns Time since last frame in seconds
    pub fn delta_time(&self) -> f64
    {
        self.last_frame.elapsed().as_secs_f64()
    }

    fn draw(&mut self, manager : &mut GameManager, delta_time : f64)
    {
        let mut target = self.display.draw();
        target.clear_color(0.1, 0.,  0.1, 1.0);
        target.finish().unwrap();
    }

    fn update(&mut self, manager : &mut GameManager, delta_time : f64)
    {

    }

    fn _window_event(&mut self, target : &EventLoopWindowTarget<()>, _window_id : WindowId, event : WindowEvent, manager : &mut GameManager)
    {
        match event
        {
            WindowEvent::RedrawRequested => 
            {
                self._frame(manager);
            },
            WindowEvent::CloseRequested =>
            {
                target.exit()
            },
            WindowEvent::Resized(size) =>
            {
                self.display.resize(size.into())
            },
            _ => {}
        }
    }    

    fn _frame_end(&mut self)
    {
        self.last_frame = Instant::now();
    }

    fn _frame(&mut self, manager : &mut GameManager)
    {
        let delta_time = self.delta_time();

        self.update(manager, delta_time);
        self.draw(manager, delta_time);

        self._frame_end()
    }
}

impl RenderAPI for OpenGL
{
    fn init_with_window(options : crate::WindowOptions) -> Self {
        let event_loop = EventLoopBuilder::new().build().unwrap();
        let (window, display) = SimpleWindowBuilder::new()
            .with_inner_size(options.size.0, options.size.1)
            .with_title(&options.title)
            .build(&event_loop);

        Self
        {
            event_loop : Some(event_loop),
            window,
            display,
            last_frame : Instant::now(),
            target_frame_rate : 60.
        }
    }

    fn take_control(mut self, mut manager : GameManager) {
        let event_loop = self.event_loop.take().unwrap();
        event_loop.run(move |event, target|
        {
            self._event_loop(event, target, &mut manager);
        }).unwrap();
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