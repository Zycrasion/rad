use glium::{index::PrimitiveType, Frame, Program};

use crate::{Assets, BakedCameraInformation, DefaultMaterial, MeshBuilder, OpenGL, Transform, Vertex};

pub(super) struct OGLMesh {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
}

impl OGLMesh {
    pub fn new(gl_context: &OpenGL, builder: MeshBuilder) -> Result<Self, ()> {
        let vertex_buffer = glium::VertexBuffer::new(&gl_context.display, &builder.vertices);
        if vertex_buffer.is_err() {
            return Err(());
        }
        let vertex_buffer = vertex_buffer.unwrap();

        let index_buffer = glium::IndexBuffer::new(
            &gl_context.display,
            PrimitiveType::TrianglesList,
            &builder.indices.unwrap_or((0..builder.vertices.len()).map(|v| v as u16).collect::<Vec<u16>>()),
        );
        if index_buffer.is_err() {
            return Err(());
        }
        let index_buffer = index_buffer.unwrap();

        Ok(Self {
            vertex_buffer,
            index_buffer,
        })
    }

    pub fn draw(&self, context : &mut Frame, transform : &Transform, baked_camera : &BakedCameraInformation, material : &DefaultMaterial, programs : &Assets<Program>) -> Result<(), glium::DrawError>
    {
        material.draw_glium(context, Some(transform), &baked_camera, (&self.vertex_buffer, &self.index_buffer), programs)
    }
}
