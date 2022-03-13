use std::sync::Arc;
use crate::resource::{Resource, ResourceCleaner};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFormat {
    SRgba8,
    Rgba32,
    R8,
    Depth16,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFilter {
    Linear,
    Nearest,
}

struct TextureRef {
    id: u64,
    cleaner: ResourceCleaner,
}

impl Drop for TextureRef {
    fn drop(&mut self) {
        self.cleaner.add(Resource::Texture(self.id));
    }
}

#[derive(Clone)]
pub struct Texture {
    id: u64,
    _ref: Arc<TextureRef>,
}
