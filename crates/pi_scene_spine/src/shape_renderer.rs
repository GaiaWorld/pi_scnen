use pi_scene_material::texture::TextureKey;
use pi_scene_math::{Number, Matrix};
use pi_scene_pipeline_key::pipeline_key::PipelineKey;

use crate::{shaders::{EShader, SpineShaderPool}, mesh::{Mesh, VertexAttribute}, pipeline::{SpinePipelinePool, SpinePipeline}};


pub struct ShapeRenderer<K2D: TextureKey> {
    pub src_factor: wgpu::BlendFactor,
    pub dst_factor: wgpu::BlendFactor,
    pub shader: EShader,
    pub meshes: Vec<Mesh<K2D>>,
    pub is_drawing: bool,
    pub draw_calls: usize,
    pub vertices_length: usize,
    pub indices_length: usize,
    pub last_texture_key: Option<K2D>,
    pub attributes: Vec<VertexAttribute>,
    pub mask_flag: (Number, Number, Number, Number),
    pub mvp_matrix: Matrix,
    pub elements_per_vertex: u32,
    pub blend: Option<wgpu::BlendState>,
    pipeline_key: Option<PipelineKey>,
    vertex_index: usize,
}

impl<K2D: TextureKey> ShapeRenderer<K2D> {
    pub fn new() -> Self {
        let attributes  = vec![
            VertexAttribute::position_2(),
            VertexAttribute::color(),
        ];
        let elements_per_vertex = VertexAttribute::elements(&attributes);
        let shader = EShader::Colored;

        Self {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::Zero,
            shader: shader,
            meshes: vec![],
            is_drawing: false,
            draw_calls: 0,
            vertices_length: 0,
            indices_length: 0,
            last_texture_key: None,
            attributes,
            mask_flag: (0., 0., 0., 0.),
            mvp_matrix: Matrix::identity(),
            elements_per_vertex,
            blend: None,
            pipeline_key: None,
            vertex_index: 0,
        }
    }

    pub fn begin<'a, SP: SpineShaderPool, SPP: SpinePipelinePool>(
        &'a mut self,
        device: & wgpu::Device,
        renderpass: &mut wgpu::RenderPass<'a>,
        target_format: wgpu::TextureFormat,
        depth_stencil: Option<wgpu::DepthStencilState>,
        shaders: &mut SP,
        pipelines: &mut SPP,
    ) {
        self.draw_calls = 0;
        self.is_drawing = true;

        let color_target = wgpu::ColorTargetState {
            format: target_format,
            blend: self.blend,
            write_mask: wgpu::ColorWrites::ALL,
        };

        let pipeline_key = SpinePipeline::check(self.shader, device, shaders, pipelines, &[color_target], wgpu::PrimitiveState::default(), depth_stencil);
        self.pipeline_key = Some(pipeline_key);
    }
    pub fn vertex(&mut self, x: Number, y: Number, r: Number, g: Number, b: Number, a: Number) {
        let mut idx = self.vertex_index;
        let mesh = self.meshes.get_mut(self.draw_calls).unwrap();
        let vertices = mesh.get_vertices_mut();
        vertices[idx] = x; idx += 1;
        vertices[idx] = y; idx += 1;
        vertices[idx] = r; idx += 1;
        vertices[idx] = g; idx += 1;
        vertices[idx] = b; idx += 1;
        vertices[idx] = a; idx += 1;
        self.vertex_index = idx;
    }
}