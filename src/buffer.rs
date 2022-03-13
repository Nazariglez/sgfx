use crate::resource::{Resource, ResourceCleaner};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

struct BufferRef {
    id: u64,
    cleaner: ResourceCleaner,
}

impl Drop for BufferRef {
    fn drop(&mut self) {
        self.cleaner.add(Resource::Buffer(self.id));
    }
}

#[derive(Clone)]
pub struct Buffer {
    id: u64,
    _ref: Arc<BufferRef>,
}

impl Buffer {
    pub(crate) fn new(id: u64, cleaner: ResourceCleaner) -> Self {
        let _ref = Arc::new(BufferRef { id, cleaner });
        Self { id, _ref }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

impl std::cmp::PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Buffer({})", self.id)
    }
}

#[cfg(test)]
mod test {
    use super::Buffer;
    use crate::resource::ResourceCleaner;

    #[test]
    fn are_equals() {
        let cleaner: ResourceCleaner = Default::default();
        let b1 = Buffer::new(0, cleaner.clone());
        let b2 = Buffer::new(0, cleaner);
        assert_eq!(b1, b2);
    }

    fn are_not_equals() {
        let cleaner: ResourceCleaner = Default::default();
        let b1 = Buffer::new(0, cleaner.clone());
        let b2 = Buffer::new(0, cleaner);
        assert_ne!(b1, b2);
    }

    fn is_cleaned_after_drop() {
        let cleaner: ResourceCleaner = Default::default();
        let buffer = Buffer::new(0, cleaner.clone());
        assert_eq!(cleaner.len(), 0);
        drop(buffer);
        assert_eq!(cleaner.len(), 1);
    }
}
