use vecto_rs::linear::Mat4;

pub(crate) struct OGLDrawCallInformation
{
    camera : Option<Mat4>,
    transform : Option<Mat4>
}