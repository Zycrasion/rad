use bevy_ecs::component::Component;

use crate::AssetHandle;

#[derive(Component)]
pub struct Mesh {
    pub handle: AssetHandle,
}

impl Clone for Mesh
{
    fn clone(&self) -> Self {
        Self { handle: self.handle.clone() }
    }
}