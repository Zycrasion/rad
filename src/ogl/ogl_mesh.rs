use glium::{backend::Facade, index::PrimitiveType, uniform, vertex, Frame, IndexBuffer, Surface};

use crate::{MeshBuilder, OpenGL, Vertex};

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

    pub fn draw(&self, context : &mut Frame, program : &glium::Program, uniforms : impl Uniform) -> Result<(), glium::DrawError>
    {
        context.draw(&self.vertex_buffer, &self.index_buffer, program, &uniform! {}, &Default::default())
    }
}
