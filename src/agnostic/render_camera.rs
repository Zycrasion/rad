use bevy_ecs::{bundle::Bundle, component::Component, query::QueryState, system::Query, world::World};
use vecto_rs::linear::Mat4;

use crate::{BakedLight, Light, Transform};

pub struct BakedCameraInformation
{
    pub params : CameraParameters,
    pub target : RenderTarget,
    pub view : [[f32; 4]; 4],
    pub projection : [[f32; 4]; 4],
    pub lights : Vec<BakedLight>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RenderTarget
{
    Window
}

#[derive(Clone, Copy)]
pub struct CameraParameters
{
    pub clear_colour : Option<(f32, f32, f32, f32)>,
}

impl Default for CameraParameters
{
    fn default() -> Self {
        Self { clear_colour: Some((0., 0., 0., 1.)) }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum ProjectionType
{
    Perspective
    {
        fov : f32,
        near : f32,
        far : f32,
    }
}

#[derive(Component)]
pub struct Camera
{
    pub render_target : RenderTarget,
    pub projection_type : ProjectionType,
    pub draw_params : CameraParameters
}

impl Camera
{
    /// Creates a Default Perspective Camera
    pub fn new() -> Self
    {
        Self::new_perspective(90., 0.1, 100.)
    }

    pub fn new_perspective(fov : f32, near : f32, far : f32) -> Self
    {
        Self
        {
            render_target: RenderTarget::Window,
            projection_type: ProjectionType::Perspective { fov, near, far },
            draw_params : Default::default()
        }
    }

    pub fn generate_projection_matrix(&self, window_size : (u32, u32)) -> [[f32; 4]; 4]
    {
        let matrix =  match self.projection_type
        {
            ProjectionType::Perspective { fov, near, far } => {
                Mat4::new_perspective_matrix(window_size.0 as f32, window_size.1 as f32, fov, near, far)
            }
        };

        unsafe { std::mem::transmute(matrix.transpose().get_contents()) }
    }

    pub fn bake(&self, eye : Option<&Transform>, window_size : (u32, u32), lights : &Vec<BakedLight>) -> BakedCameraInformation
    {


        BakedCameraInformation
        {
            params: self.draw_params,
            target: self.render_target,
            view: eye.unwrap_or(&Transform::new()).as_uniform_inverse(),
            projection: self.generate_projection_matrix(window_size),
            lights : lights.clone()
        }
    }
}

#[derive(Bundle)]
pub struct CameraBundle
{
    pub transform : Transform,
    pub camera : Camera
}

impl CameraBundle
{
    pub fn new() -> Self
    {
        Self { transform: Transform::new(), camera: Camera::new() }
    }
}