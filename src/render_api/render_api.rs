use bevy_ecs::component::Component;

use crate::{AssetHandle, GameManager, MeshBuilder, WindowOptions};



#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ControlFlow
{
    Continue,
    Exit,
}

#[derive(Clone, Copy)]
pub enum WindowEvents
{
    MouseClick(u8, (f32, f32)),
    KeyDown(char),
    KeyUp(char)
}

pub trait RenderAPI
{
    fn init_with_window(options : WindowOptions) -> Self;
    fn take_control(self, manager : GameManager);
    
    fn log_error(&self, message : &str);
    fn log_debug(&self, message : &str);

    fn create_mesh(&mut self, mesh_builder : MeshBuilder) -> AssetHandle;
}