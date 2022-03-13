#[derive(Debug, Clone, Copy)]
pub enum VertexFormat {
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,
    UInt8,
    UInt8Norm,
    UInt8x2,
    UInt8x2Norm,
    UInt8x3,
    UInt8x3Norm,
    UInt8x4,
    UInt8x4Norm,
}

impl VertexFormat {
    pub fn size(&self) -> i32 {
        match self {
            VertexFormat::Float32 => 1,
            VertexFormat::Float32x2 => 2,
            VertexFormat::Float32x3 => 3,
            VertexFormat::Float32x4 => 4,
            VertexFormat::UInt8 => 1,
            VertexFormat::UInt8Norm => 1,
            VertexFormat::UInt8x2 => 2,
            VertexFormat::UInt8x2Norm => 2,
            VertexFormat::UInt8x3 => 3,
            VertexFormat::UInt8x3Norm => 3,
            VertexFormat::UInt8x4 => 4,
            VertexFormat::UInt8x4Norm => 4,
        }
    }

    pub fn bytes(&self) -> i32 {
        use VertexFormat::*;
        match &self {
            UInt8 | UInt8x2 | UInt8x3 | UInt8x4 => self.size(),
            UInt8Norm | UInt8x2Norm | UInt8x3Norm | UInt8x4Norm => self.size(),
            _ => self.size() * 4,
        }
    }

    pub fn normalized(&self) -> bool {
        use VertexFormat::*;
        match &self {
            UInt8Norm | UInt8x2Norm | UInt8x3Norm | UInt8x4Norm => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VertexAttr {
    pub location: u32,
    pub format: VertexFormat,
}

impl VertexAttr {
    pub fn new(location: u32, vertex_data: VertexFormat) -> Self {
        Self {
            location,
            format: vertex_data,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VertexStepMode {
    Vertex,
    Instance,
}

impl Default for VertexStepMode {
    fn default() -> Self {
        VertexStepMode::Vertex
    }
}

#[derive(Clone, Debug, Default)]
pub struct VertexInfo {
    pub(crate) attrs: Vec<VertexAttr>,
    pub(crate) step_mode: VertexStepMode,
}

impl VertexInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attr(mut self, location: u32, format: VertexFormat) -> Self {
        self.attrs.push(VertexAttr::new(location, format));
        self
    }

    pub fn step_mode(mut self, mode: VertexStepMode) -> Self {
        self.step_mode = mode;
        self
    }
}

#[cfg(test)]
mod test {
    use crate::vertex::VertexFormat::*;

    #[test]
    fn u8_matches_size_and_bytes() {
        let types = [
            UInt8,
            UInt8Norm,
            UInt8x2,
            UInt8x2Norm,
            UInt8x3,
            UInt8x3Norm,
            UInt8x4,
            UInt8x4Norm,
        ];

        types
            .iter()
            .for_each(|vf| assert_eq!(vf.size(), vf.bytes()));
    }

    #[test]
    fn f32_bytes_are_size_x_4() {
        let types = [Float32, Float32x2, Float32x3, Float32x4];

        types
            .iter()
            .for_each(|vf| assert_eq!(vf.size() * 4, vf.bytes()));
    }

    #[test]
    fn u8_norm_are_normalized_types() {
        let normalized = [UInt8Norm, UInt8x2Norm, UInt8x3Norm, UInt8x4Norm];

        normalized.iter().for_each(|vf| assert!(vf.normalized()));
    }

    #[test]
    fn non_norm_types_are_not_normalized_types() {
        let normalized = [
            UInt8, UInt8x2, UInt8x3, UInt8x4, Float32, Float32x2, Float32x3, Float32x4,
        ];

        normalized.iter().for_each(|vf| assert!(!vf.normalized()));
    }
}
