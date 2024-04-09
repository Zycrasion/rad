use bevy_ecs::{bundle::Bundle, component::Component};
use vecto_rs::linear::Mat4;

use crate::Transform;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderTarget
{
    Viewport
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
    pub projection_type : ProjectionType
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
            render_target: RenderTarget::Viewport,
            projection_type: ProjectionType::Perspective { fov, near, far },
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