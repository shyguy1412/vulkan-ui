mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shader/vertex.glsl"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shader/fragment.glsl"
    }
}
