use bevy_ecs::component::Component;
use glium::framebuffer::StencilRenderBuffer;
use vecto_rs::linear::{Mat4, Vector, Vector4, VectorTrait};

#[derive(Component)]
pub struct Transform {
    pub position: Vector,
    pub rotation: Vector,
    pub scale: f32,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vector::new2(0., 0.),
            rotation: Vector::new2(0., 0.),
            scale: 1.,
        }
    }

    pub fn into_matrix(&self) -> Mat4
    {
        let mut matrix = Mat4::identity();
        matrix.rotate(self.rotation.x, Vector::new3(1., 0. , 0.));
        matrix.rotate(self.rotation.y, Vector::new3(0., 1. , 0.));
        matrix.rotate(self.rotation.z, Vector::new3(0., 0. , 1.));
        matrix.translate(self.position);
        matrix
    }

    pub fn as_uniform(&self) -> [[f32; 4]; 4]
    {
        let contents = self.into_matrix().get_contents();
        // I Couldn't Think of any other Peformant Ways 
        unsafe { std::mem::transmute(contents) }
    }
}
