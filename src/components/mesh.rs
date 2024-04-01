use bevy_ecs::component::Component;

use crate::AssetHandle;

#[derive(Component)]
pub struct Mesh {
    pub handle: AssetHandle,
}