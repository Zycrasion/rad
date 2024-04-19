use bevy_ecs::system::{NonSend, Query, Res, Resource};
use glium::{Frame, Program};

use crate::{Assets, Material, Mesh, OpenGL, RenderAPI, Transform};
use crate::ogl::OGLMesh;

// pub(crate) fn ogl_draw_with_material<T : Material>(query : Query<(&Mesh, &T, Option<&Transform>)>, meshes : NonSend<Assets<OGLMesh>>, programs : NonSend<Assets<Program>>, context : NonSend<(&mut Frame,)>)
// {
//     for (mesh_handle, material, transform) in query
//     {
//         let mesh = meshes.get_asset(&mesh_handle.handle).unwrap();
//         let draw_result = mesh.draw(*context, transform, baked_camera, material, programs)
//     }
// }