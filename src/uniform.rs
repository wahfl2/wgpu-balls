use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VpSizeUniform {
    pub(crate) viewport_size: [f32; 2],
}

impl VpSizeUniform {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        Self { viewport_size: size.into() }
    }
}