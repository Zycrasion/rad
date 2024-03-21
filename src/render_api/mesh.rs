pub struct MeshHandle
{
    pub(super) index : u32,
    pub(super) magic : u32
}

pub struct Mesh
{
    pub(crate) handle : MeshHandle
}