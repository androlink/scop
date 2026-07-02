use std::{borrow::Borrow, collections::HashMap, hash::Hash, sync::Arc};

#[derive(Default)]
pub struct Storage<StorageId, T> {
    container: HashMap<StorageId, Arc<T>>,
}

impl<StorageId, T> Storage<StorageId, T>
where
    StorageId: Eq + Hash,
{
    pub fn get<StorageKey>(&self, id: &StorageKey) -> Option<&Arc<T>>
    where
        StorageId: Borrow<StorageKey>,
        StorageKey: Eq + Hash + ?Sized,
    {
        self.container.get(id)
    }
    pub fn has<StorageKey>(&self, id: &StorageKey) -> bool
    where
        StorageId: Borrow<StorageKey>,
        StorageKey: Eq + Hash + ?Sized,
    {
        self.container.contains_key(id)
    }

    pub fn insert(&mut self, id: StorageId, item: T) -> Option<Arc<T>> {
        self.container.insert(id, Arc::new(item))
    }

    pub fn remove<StorageKey>(&mut self, id: &StorageKey)
    where
        StorageId: Borrow<StorageKey>,
        StorageKey: Eq + Hash + ?Sized,
    {
        self.container.remove(id);
    }
}
