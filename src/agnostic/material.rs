use std::sync::Mutex;

use crate::{
    AssetHandle, Assets, BakedCameraInformation, Colour, OpenGL, RenderAPI, Transform, Vertex,
};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, Query},
    world::World,
};
use glium::{
    backend::Backend,
    glutin::surface::WindowSurface,
    program::{self, ShaderStage},
    uniform,
    uniforms::{UniformBuffer, Uniforms, UniformsStorage},
    Display, Frame, IndexBuffer, Program, Surface, VertexBuffer,
};


pub trait Material: Component + Sync + Send + Sized {
    fn glsl() -> (&'static str, &'static str);
}

#[derive(Component, Clone)]
pub struct DefaultMaterial {
    pub shading_enabled: bool,

    pub base_colour: Colour,
}

impl Default for DefaultMaterial {
    fn default() -> Self {
        Self {
            shading_enabled: true,
            base_colour: Colour::WHITE,
        }
    }
}

static MATERIAL_SHADER_HANDLE: Mutex<Option<AssetHandle>> = Mutex::new(None);
impl DefaultMaterial {
    const VS_SOURCE: &'static str = r#"
    #version 400

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;

    attribute vec3 position;
    attribute vec3 normal;
    attribute vec2 uv;

    void main()
    {
        gl_Position = projection * (view * model * vec4(position, 1.0));
    }
"#;
    const FS_SOURCE: &'static str = r#"
    #version 400
    #extension GL_ARB_shader_subroutine : require

    uniform vec3 base_colour;
    uniform vec3 light_colour;

    subroutine vec3 shading();
    subroutine uniform shading shade;

    subroutine(shading) vec3 shading_enabled()
    {
        return base_colour;
    }

    subroutine(shading) vec3 shading_disabled()
    {
        return base_colour;
    }

    void main()
    {
        gl_FragColor = vec4(shade(), 1.0);
    }
"#;

    pub(crate) fn glium_register(display: &Display<WindowSurface>, shaders: &mut Assets<Program>) {
        if MATERIAL_SHADER_HANDLE.lock().unwrap().is_some() {
            return;
        }

        let program =
            Program::from_source(display, Self::VS_SOURCE, Self::FS_SOURCE, None).unwrap();

        *MATERIAL_SHADER_HANDLE.lock().unwrap() = Some(shaders.add_asset(program));
    }

    pub fn new(base_colour: Colour) -> DefaultMaterial {
        DefaultMaterial {
            base_colour,
            ..Default::default()
        }
    }

    pub fn draw_glium(
        &self,
        context: &mut Frame,
        transform: Option<&Transform>,
        baked_camera: &BakedCameraInformation,
        buffers: (&VertexBuffer<Vertex>, &IndexBuffer<u16>),
        programs: &Assets<Program>,
    ) -> Result<(), glium::DrawError> {
        context.draw(
            buffers.0,
            buffers.1,
            programs
                .get_asset(&MATERIAL_SHADER_HANDLE.lock().unwrap().unwrap())
                .unwrap(),
            &uniform! {
                model : transform.unwrap_or(&Transform::new()).as_uniform(),
                projection : baked_camera.projection,
                view : baked_camera.view,

                base_colour : self.base_colour,
                light_colour : baked_camera.lights.get(0).map(|v| v.light.colour).unwrap_or(Colour::WHITE),

                shade: (if self.shading_enabled {"shading_enabled"} else {"shading_disabled"}, ShaderStage::Fragment)
            },
            &OpenGL::default_draw_params(),
        )
    }

    // pub fn default_component() -> Material<DefaultMaterial> {
    //     Material {
    //         material: DefaultMaterial::default(),
    //     }
    // }
}

impl Material for DefaultMaterial {
    fn glsl() -> (&'static str, &'static str) {
        (Self::VS_SOURCE, Self::FS_SOURCE)
    }
}
