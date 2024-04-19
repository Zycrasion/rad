use std::marker::PhantomData;

use crate::AssetHandle;

pub struct ShaderSource<T>
{
    pub vertex_source : String,
    pub fragment_source : String,
    _ty : PhantomData<T> // Compile Time Enforcement for correct shading languages
}

pub struct Shader
{
    pub handle : AssetHandle
}