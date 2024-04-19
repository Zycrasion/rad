use bevy_ecs::component::Component;
use winit::{event_loop::EventLoop, window::{Window, WindowBuilder}};

use crate::{Mesh, MeshBuilder, Shader, ShaderSource, GLSL};




#[derive(Clone, Copy)]
pub enum WindowEvents
{
    MouseClick(u8, (f32, f32)),
    KeyDown(char),
    KeyUp(char)
}

pub trait RenderAPI
{
    fn create_mesh(&mut self, mesh_builder : MeshBuilder) -> Mesh;
    fn create_program(&mut self, shader : &ShaderSource<GLSL>)  -> Shader;
}