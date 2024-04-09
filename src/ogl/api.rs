use std::{collections::HashMap, fs::File, hash::Hash, io::Write, rc::Rc, sync::Mutex, thread, time::{Duration, Instant}};

use bevy_ecs::{query::QueryState, system::{Query, Resource}};
use glium::{backend::glutin::{self, SimpleWindowBuilder}, glutin::{config::{self, ConfigTemplateBuilder}, surface::WindowSurface}, program, Display, Program, Surface};
use vecto_rs::linear::Mat4;
use winit::{dpi::{PhysicalSize, Size}, event::{Event, WindowEvent}, event_loop::{self, EventLoop, EventLoopBuilder, EventLoopWindowTarget}, window::{Window, WindowBuilder, WindowId}};
use glium::glutin::prelude::*;
use crate::{ogl::OGLMesh, Assets, BakedCameraInformation, Camera, GameManager, Mesh, RenderAPI, Transform};

const API_NAME : &str = "OpenGL4";

pub struct OpenGL
{
    pub(crate) event_loop : Option<EventLoop<()>>,
    pub(crate) window : Window,
    pub(crate) display : Display<WindowSurface>,
    pub(crate) last_frame : Instant,
    pub(crate) target_frame_rate : f64,

    pub(crate) meshes : Assets<OGLMesh>,

    pub(crate) default_program : Program,
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
                if self.delta_time() > 1. / self.target_frame_rate && !manager.finished_running()
                {
                    self.window.request_redraw();
                }
                target.set_control_flow(event_loop::ControlFlow::Poll);
            },
            _ => {}
        }
    }

    /// Returns Time since last frame in seconds
    pub fn delta_time(&self) -> f64
    {
        self.last_frame.elapsed().as_secs_f64()
    }

    fn update(&mut self, manager : &mut GameManager, delta_time : f64)
    {
        manager.step_update();
    }

    fn draw(&mut self, manager : &mut GameManager, baked_camera : BakedCameraInformation)
    {
        let mut target = match baked_camera.target
        {
            crate::RenderTarget::Window => self.display.draw(),
        };

        if let Some(camera_clear) = baked_camera.params.clear_colour
        {
            target.clear_color_and_depth(camera_clear, 1.0);
        } else
        {
            target.clear_depth(1.0);
        }

        
        let mut meshes : QueryState<(&Mesh, &Transform)> = manager.world.query();

        for (mesh_component, transform) in meshes.iter(&manager.world)
        {
            let mesh = self.meshes.get_asset(&mesh_component.handle);

            if mesh.is_none() {self.log_debug("Invalid Mesh Handle"); continue;}

            let mesh = mesh.unwrap();

            let result = mesh.draw(&mut target, &self.default_program, transform, &baked_camera);

            if result.is_err()
            {
                self.log_error(&format!("glium::DrawError - {}", result.unwrap_err()))
            }
        }


        target.finish().unwrap();
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
                self.log_debug("Close Requested");
                manager.end();
                target.exit()
            },
            WindowEvent::Resized(size) =>
            {
                self.log_debug("Viewport Resized");
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

        manager.step_draw();
        let mut cameras : QueryState<(&Camera, Option<&Transform>)> = manager.world.query();
        let mut baked_camera_information : Vec<BakedCameraInformation> = Vec::new();

        let window_size = self.window.inner_size();
        let window_size = (window_size.width, window_size.height);
        for (camera, eye) in cameras.iter(&manager.world)
        {
            baked_camera_information.push(camera.bake(eye, window_size));
        }

        for baked_camera in baked_camera_information
        {
            self.draw(manager, baked_camera);
        }

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

        let default_program = program!(
            &display,
            100 => {
                vertex : "
                    #version 100

                    uniform lowp mat4 model;
                    uniform lowp mat4 view;
                    uniform lowp mat4 projection;

                    attribute lowp vec3 position;
                    attribute lowp vec3 normal;
                    attribute lowp vec2 uv;

                    varying lowp vec3 vert_colour;

                    void main()
                    {
                        gl_Position = projection * (view * model * vec4(position, 1.0));
                        vert_colour = normal;
                    }
                ",
                fragment : "
                    #version 100

                    varying lowp vec3 vert_colour;
                    void main()
                    {
                        gl_FragColor = vec4(vert_colour, 1.0);
                    }
                ",
            }
        ).unwrap();

        Self
        {
            event_loop : Some(event_loop),
            window,
            display,
            last_frame : Instant::now(),
            target_frame_rate : 60.,
            meshes : Assets::new(),
            default_program
        }
    }

    fn inject_systems(&self, manager : &mut GameManager) {
        
    }

    fn take_control(mut self, mut manager : GameManager) {
        self.log_debug("Main Event Loop Started");
        manager.run_startup();
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
    
    fn create_mesh(&mut self, mesh_builder : crate::MeshBuilder) -> crate::AssetHandle {
        let mesh = OGLMesh::new(&self, mesh_builder).unwrap();
        self.meshes.add_asset(mesh)
    }
}