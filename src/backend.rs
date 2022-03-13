use crate::vertex::{VertexAttr, VertexStepMode};

/// Represents a the implementation graphics backend like glow, wgpu or another
pub trait Backend {
    /// Returns the name of the api used (like webgl, wgpu, etc...)
    fn api_name(&self) -> &str;

    /// Return the device limits
    fn limits(&self) -> Limits {
        Default::default()
    }

    /// Create a new pipeline and returns the id
    fn create_pipeline(
        &mut self,
        vertex_source: &[u8],
        fragment_source: &[u8],
        vertex_attrs: &[VertexAttr],
        options: PipelineOptions,
    ) -> Result<u64, String>;

    /// Create a new vertex buffer object and returns the id
    fn create_vertex_buffer(
        &mut self,
        attrs: &[VertexAttr],
        step_mode: VertexStepMode,
    ) -> Result<u64, String>;

    /// Create a new index buffer object and returns the id
    fn create_index_buffer(&mut self) -> Result<u64, String>;

    /// Create a new uniform buffer and returns the id
    fn create_uniform_buffer(&mut self, slot: u32, name: &str) -> Result<u64, String>;

    /// Upload to the GPU the buffer data slice
    fn set_buffer_data(&mut self, buffer: u64, data: &[u8]);

    /// Create a new renderer using the size of the graphics
    fn render(&mut self, commands: &[Commands], target: Option<u64>);

    /// Clean all the dropped resources
    fn clean(&mut self, to_clean: &[ResourceId]);

    /// Sets the render size
    fn set_size(&mut self, width: i32, height: i32);

    /// Sets the screen dpi
    fn set_dpi(&mut self, scale_factor: f64);

    /// Create a new texture and returns the id
    fn create_texture(&mut self, info: &TextureInfo) -> Result<u64, String>;

    /// Create a new render target and returns the id
    fn create_render_texture(&mut self, texture_id: u64, info: &TextureInfo)
                             -> Result<u64, String>;

    /// Update texture data
    fn update_texture(&mut self, texture: u64, opts: &TextureUpdate) -> Result<(), String>;

    /// Read texture pixels
    fn read_pixels(
        &mut self,
        texture: u64,
        bytes: &mut [u8],
        opts: &TextureRead,
    ) -> Result<(), String>;
}
