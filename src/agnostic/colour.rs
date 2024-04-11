use glium::uniforms::AsUniformValue;

#[derive(Clone, Copy)]
pub struct Colour(pub f32, pub f32, pub f32);

impl Colour
{
    pub const WHITE : Colour        = Colour(1., 1., 1.);
    pub const BLACK : Colour        = Colour(0., 0., 0.);
    pub const RED : Colour          = Colour(1., 0., 0.);
    pub const GREEN : Colour        = Colour(0., 1., 0.);
    pub const BLUE : Colour         = Colour(0., 0., 1.);
    pub const TRANSPARENT : Colour  = Colour(0., 0., 0.);
}

impl AsUniformValue for Colour
{
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue<'_> {
        glium::uniforms::UniformValue::Vec3([self.0, self.1, self.2])
    }
}