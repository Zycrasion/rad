use std::{any::TypeId, collections::HashMap, default, f32::consts::E, process::exit, time::Instant};

use bevy_ecs::{
    bundle::Bundle,
    schedule::{ExecutorKind, IntoSystemConfigs, Schedule, ScheduleLabel},
    world::{EntityWorldMut, World},
};
use winit::{dpi::LogicalSize, event::WindowEvent, event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget}, window::{Window, WindowBuilder}};

use crate::*;

// TODO: V-Sync and other options
pub const FRAME_RATE : f32 = 60.;

#[derive(Resource)]
pub struct DeltaTime {
    pub delta_time: f64,
}

#[derive(ScheduleLabel, Default, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ScheduleTimes {
    #[default]
    Startup,
    Update,
    Draw,
    End,
}

pub use ScheduleTimes::*;

#[derive(Resource, Clone)]
pub struct WindowResource
{
    size : (u32, u32),
    title : String
}

pub struct App {
    api: Box<OpenGL>,
    pub world: World,
    pub(crate) schedules: HashMap<ScheduleTimes, Schedule>,
    finished_running : bool,

    window : Window,
    event_loop : Option<EventLoop<()>>,

    last_frame : Instant
}

impl App {
    pub fn with_window_builder(builder : WindowBuilder) -> Self
    {
        let mut schedules = HashMap::new();
        // TODO: Make a system that is more viable than this
        for time in vec![Startup, Update, Draw, End] {
            schedules.insert(time, Schedule::new(time));
        }

        schedules.get_mut(&Draw).unwrap().set_executor_kind(ExecutorKind::Simple);

        let (window, event_loop, api) = OpenGL::init(builder);

        let mut world = World::new();
        world.insert_resource(WindowResource {size : (0,0 ), title : "".to_string()});

        App {
            api : Box::new(api),
            window,
            event_loop : Some(event_loop),
            world,
            schedules,
            finished_running : false,
            last_frame : Instant::now()
        }
    }

    pub fn new() -> Self {
        Self::with_window_builder(
            WindowBuilder::new()
                .with_inner_size(LogicalSize::new(480, 480))
                .with_title("Rad Engine Test")
        )
    }

    pub fn delta_time(&self) -> f32
    {
        self.last_frame.elapsed().as_secs_f32()
    }

    pub fn run(mut self) -> ! {
        let event_loop = self.event_loop.take().unwrap();
        event_loop.run(move |event, target| {
            match event
            {
                winit::event::Event::WindowEvent { window_id, event } => self.window_event(event, target),
                winit::event::Event::AboutToWait => {
                    if self.delta_time() > 1. / FRAME_RATE
                    {
                        self.window.request_redraw();
                    }

                    target.set_control_flow(ControlFlow::Poll)
                },
                _ => {}
            }
        }).unwrap();
        exit(0);
    }

    fn window_event(&mut self, window_event : WindowEvent, target : &EventLoopWindowTarget<()>)
    {
        match window_event
        {
            WindowEvent::Resized(size) => 
            {
                self.world.get_resource_mut::<WindowResource>().unwrap().size = (size.width, size.height);
            },
            WindowEvent::CloseRequested => 
            {
                target.exit();
            },
            WindowEvent::RedrawRequested => 
            {
                let mut cameras: QueryState<(&Camera, Option<&Transform>)> = self.world.query();
                let mut baked_camera_information: Vec<BakedCameraInformation> = Vec::new();

                let window_size = self.window.inner_size();
                let window_size = (window_size.width, window_size.height);
                for (camera, eye) in cameras.iter(&self.world) {
                    baked_camera_information.push(camera.bake(eye, window_size, &vec![]));
                }

                for cam in baked_camera_information
                {
                    self.api.draw(&mut self.world, &cam);
                }

                self.schedules.get_mut(&Draw).unwrap().run(&mut self.world);
                self.last_frame = Instant::now();
            },
            _ => {}
        }
    }

    pub fn register_mesh(&mut self, builder : MeshBuilder) -> Mesh
    {
        self.api.create_mesh(builder)
    }

    pub fn run_startup(&mut self) {
        if let Some(mut startup) = self.schedules.remove(&Startup)
        {
            startup.run(&mut self.world);
        }
    }

    pub fn end(&mut self) {
        if let Some(mut end) = self.schedules.remove(&End)
        {
            end.run(&mut self.world);
            self.finished_running = true;
        }
    }

    pub fn step_draw(&mut self) {
        self.schedules
            .get_mut(&Draw)
            .unwrap()
            .run(&mut self.world);
    }

    pub fn step_update(&mut self) {
        self.schedules
            .get_mut(&Update)
            .unwrap()
            .run(&mut self.world);
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut {
        self.world.spawn(bundle)
    }

    pub fn add_systems<M>(
        &mut self,
        time: ScheduleTimes,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        self.schedules.get_mut(&time).unwrap().add_systems(systems);
        self
    }

    pub fn finished_running(&self) -> bool
    {
        self.finished_running
    }
}
