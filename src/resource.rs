use parking_lot::RwLock;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Resource {
    Buffer(u64),
    Texture(u64),
    Pipeline(u64),
    RenderTexture(u64),
}

#[derive(Default, Debug, Clone)]
pub(crate) struct ResourceCleaner {
    to_clean: Arc<RwLock<HashSet<Resource>>>,
}

impl ResourceCleaner {
    pub fn add(&self, resource: Resource) {
        self.to_clean.write().insert(resource);
    }

    pub fn reset(&self) {
        self.to_clean.write().clear();
    }

    pub fn len(&self) -> usize {
        self.to_clean.read().len()
    }
}

#[cfg(test)]
mod test {
    use super::Resource;
    use super::ResourceCleaner;

    #[test]
    fn add_resource() {
        let cleaner = ResourceCleaner::default();
        assert_eq!(cleaner.len(), 0);
        cleaner.add(Resource::Buffer(0));
        assert_eq!(cleaner.len(), 1);
    }

    #[test]
    fn do_not_add_resource_twice() {
        let cleaner = ResourceCleaner::default();
        assert_eq!(cleaner.len(), 0);
        cleaner.add(Resource::Buffer(0));
        assert_eq!(cleaner.len(), 1);
        cleaner.add(Resource::Buffer(0));
        assert_eq!(cleaner.len(), 1);
    }

    #[test]
    fn clear_list() {
        let cleaner = ResourceCleaner::default();
        let ids = [0, 1, 2, 3, 4];
        ids.iter().for_each(|n| cleaner.add(Resource::Buffer(*n)));
        assert_eq!(cleaner.len(), ids.len());
        cleaner.reset();
        assert_eq!(cleaner.len(), 0);
    }
}
