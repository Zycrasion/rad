use crate::{
    ogl::OGLMesh, Assets, BakedCameraInformation, DefaultMaterial, Material, Mesh, RenderAPI, Transform
};
use bevy_ecs::{query::QueryState, world::World};
use glium::{
    backend::glutin::SimpleWindowBuilder, glutin::
        surface::WindowSurface, Depth, DepthTest, Display, DrawParameters, Program, Surface
};
use winit::{
    event_loop::{EventLoop, EventLoopBuilder},
    window::{Window, WindowBuilder},
};

const API_NAME: &str = "OpenGL4";

pub struct OpenGL {
    pub(crate) display: Display<WindowSurface>,
    meshes: Assets<OGLMesh>,
    pub(crate) shaders: Assets<Program>,
}

impl OpenGL {
    pub fn default_draw_params() -> DrawParameters<'static> {
        DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    // fn _event_loop(
    //     &mut self,
    //     event: Event<()>,
    //     target: &EventLoopWindowTarget<()>,
    //     manager: &mut GameManager,
    // ) {
    //     match event {
    //         Event::WindowEvent { window_id, event } => {
    //             self._window_event(target, window_id, event, manager)
    //         }
    //         Event::AboutToWait => {
    //             if self.delta_time() > 1. / self.target_frame_rate && !manager.finished_running() {
    //                 self.window.request_redraw();
    //             }
    //             target.set_control_flow(event_loop::ControlFlow::Poll);
    //         }
    //         _ => {}
    //     }
    // }

    // /// Returns Time since last frame in seconds
    // pub fn delta_time(&self) -> f64 {
    //     self.last_frame.elapsed().as_secs_f64()
    // }

    // fn update(&mut self, manager: &mut GameManager, delta_time: f64) {
    //     manager.step_update();
    // }

    pub fn draw(&mut self, world :  &mut World, baked_camera : &BakedCameraInformation) {
        let mut target = match baked_camera.target {
            crate::RenderTarget::Window => self.display.draw(),
        };

        if let Some(camera_clear) = baked_camera.params.clear_colour {
            target.clear_color_and_depth(camera_clear, 1.0);
        } else {
            target.clear_depth(1.0);
        }

        let mut meshes: QueryState<(&Mesh, &Transform, &DefaultMaterial)> = world.query();

        for (mesh_component, transform, material) in meshes.iter(&world) {
            let mesh = self.meshes.get_asset(&mesh_component.handle);

            if mesh.is_none() {
                continue;
            }

            let mesh = mesh.unwrap();

            let result = mesh.draw(
                &mut target,
                transform,
                &baked_camera,
                material,
                &self.shaders,
            );

            if result.is_err() {
                println!("glium::DrawError - {}", result.unwrap_err());
            }
        }

        target.finish().unwrap();
    }

    // fn _window_event(
    //     &mut self,
    //     target: &EventLoopWindowTarget<()>,
    //     _window_id: WindowId,
    //     event: WindowEvent,
    //     manager: &mut GameManager,
    // ) {
    //     match event {
    //         WindowEvent::RedrawRequested => {
    //             self._frame(manager);
    //         }
    //         WindowEvent::CloseRequested => {
    //             self.log_debug("Close Requested");
    //             manager.end();
    //             target.exit()
    //         }
    //         WindowEvent::Resized(size) => {
    //             self.log_debug("Viewport Resized");
    //             self.display.resize(size.into())
    //         }
    //         _ => {}
    //     }
    // }

    // fn _frame_end(&mut self) {
    //     self.last_frame = Instant::now();
    // }

    // fn _frame(&mut self, manager: &mut GameManager) {
    //     let delta_time = self.delta_time();

    //     self.update(manager, delta_time);

    //     manager.step_draw();

    //     let mut lights_query: QueryState<(&Light, &Transform)> = manager.world.query();
    //     let mut lights = vec![];

    //     for (light, transform) in lights_query.iter(&manager.world) {
    //         lights.push(BakedLight {
    //             transform: *transform,
    //             light: *light,
    //         })
    //     }

    //     let mut cameras: QueryState<(&Camera, Option<&Transform>)> = manager.world.query();
    //     let mut baked_camera_information: Vec<BakedCameraInformation> = Vec::new();

    //     let window_size = self.window.inner_size();
    //     let window_size = (window_size.width, window_size.height);
    //     for (camera, eye) in cameras.iter(&manager.world) {
    //         baked_camera_information.push(camera.bake(eye, window_size, &lights));
    //     }

    //     for baked_camera in baked_camera_information {
    //         self.draw(manager, baked_camera);
    //     }

    //     self._frame_end()
    // }

    pub fn init(window_builder: WindowBuilder) -> (Window, EventLoop<()>, Self) {
        let event_loop = EventLoopBuilder::new().build().unwrap();
        let (window, display) = SimpleWindowBuilder::new().set_window_builder(window_builder).build(&event_loop);

        let mut shaders = Assets::new();

        DefaultMaterial::glium_register(&display, &mut shaders);


        (window, event_loop, Self {display, meshes : Assets::new(), shaders})
    }
}

impl RenderAPI for OpenGL {
    fn create_program(&mut self, shader : &crate::ShaderSource<crate::GLSL>)  -> crate::Shader {
        let shader = Program::from_source(&self.display, &shader.vertex_source, &shader.fragment_source, None).unwrap();
        let shader = self.shaders.add_asset(shader);
        crate::Shader { handle: shader }
    }

    fn create_mesh(&mut self, mesh_builder: crate::MeshBuilder) -> crate::Mesh {
        let mesh = OGLMesh::new(&self, mesh_builder).unwrap();
        crate::Mesh { handle:  self.meshes.add_asset(mesh)}
    }
}
