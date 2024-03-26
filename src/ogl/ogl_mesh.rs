use glium::{index::PrimitiveType, vertex};

use crate::{MeshBuilder, OpenGL, Vertex};

pub(super) struct OGLMesh {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: Option<glium::IndexBuffer<u16>>,
}

impl OGLMesh {
    pub fn new(gl_context: &OpenGL, builder: MeshBuilder) -> Result<Self, ()> {
        let vertex_buffer = glium::VertexBuffer::new(&gl_context.display, &builder.vertices);
        if vertex_buffer.is_err() {
            return Err(());
        }
        let vertex_buffer = vertex_buffer.unwrap();

        let index_buffer = if let Some(indices) = builder.indices {
            // TODO: Change primitive type to be configurable
            let index_buffer = glium::IndexBuffer::new(
                &gl_context.display,
                PrimitiveType::TrianglesList,
                &indices,
            );
            if index_buffer.is_err() {
                return Err(());
            }
            Some(index_buffer.unwrap())
        } else {
            None
        };

        Ok(Self {
            vertex_buffer,
            index_buffer,
        })
    }
}
