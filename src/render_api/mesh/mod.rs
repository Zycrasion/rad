use std::io::{BufReader, Cursor};

use bevy_ecs::component::Component;
use glium::implement_vertex;

use crate::AssetHandle;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

implement_vertex!(Vertex, position, normal, uv);

pub struct MeshBuilder {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u16>>,
}

impl MeshBuilder {
    pub fn from_obj<S: AsRef<str>>(contents: S) -> MeshBuilder {
        let mut obj = prospect_obj::parse_obj(contents);

        let mut verts = Vec::with_capacity(obj.vertices.len());

        for (pos, tex, norm) in obj.extract_vertices_and_uv_and_normals()
        {
            verts.push(
                Vertex
                {
                    position : [pos.x, pos.y, pos.z],
                    normal : [norm.x, norm.y, norm.z],
                    uv : [tex.x, tex.y]
                }
            )
        }

        MeshBuilder
        {
            vertices: verts,
            indices: None,
        }
    }
}