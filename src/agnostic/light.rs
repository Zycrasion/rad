use bevy_ecs::component::Component;

use crate::{Colour, Transform};

#[derive(Component, Clone, Copy)]
pub struct Light
{
    pub colour : Colour
}


#[derive(Clone, Copy)]
pub struct BakedLight
{
    pub transform : Transform,
    pub light : Light
}