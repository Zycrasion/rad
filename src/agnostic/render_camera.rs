use bevy_ecs::{bundle::Bundle, component::Component};

use crate::Transform;

pub enum RenderTarget
{
    Viewport
}

#[derive(Component)]
pub struct Camera
{
    pub render_target : RenderTarget,
}

#[derive(Bundle)]
pub struct CameraBundle
{
    transform : Transform,
    camera : Camera
}