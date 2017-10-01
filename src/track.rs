
use std::marker::PhantomData;

use specs::{self, Entity, Join, Index, UnprotectedStorage};
use hibitset::{BitSet, BitSetLike};

pub struct TrackStorage<C, T> {
    mask: BitSet,
    inserted: BitSet,
    removed: BitSet,
    storage: T,
    phantom: PhantomData<C>,
}

impl<C, T: Default> Default for TrackStorage<C, T> {
    fn default() -> Self {
        TrackStorage {
            mask: BitSet::new(),
            inserted: BitSet::new(),
            removed: BitSet::new(),

            storage: T::default(),
            phantom: PhantomData,
        }
    }
}

impl<C, T: UnprotectedStorage<C>> UnprotectedStorage<C> for TrackStorage<C, T> {
    unsafe fn clean<F>(&mut self, has: F)
        where F: Fn(Index) -> bool
    {
        self.mask.clear();
        self.inserted.clear();
        self.removed.clear();
        self.storage.clean(has);
    }
    unsafe fn get(&self, id: Index) -> &C {
        self.storage.get(id)
    }
    unsafe fn get_mut(&mut self, id: Index) -> &mut C {
        // calling `.iter()` on an unconstrained mutable storage will flag everything
        self.mask.add(id);
        self.storage.get_mut(id)
    }
    unsafe fn insert(&mut self, id: Index, comp: C) {
        self.mask.add(id);
        self.inserted.add(id);
        self.storage.insert(id, comp);
    }
    unsafe fn remove(&mut self, id: Index) -> C {
        self.mask.remove(id);
        self.removed.add(id);
        self.storage.remove(id)
    }
}

impl<C, T: UnprotectedStorage<C>> TrackStorage<C, T> {
    pub fn inserted(&self) -> &BitSet {
        &self.inserted
    }
    pub fn removed(&self) -> &BitSet {
        &self.removed
    }
    /// Whether the component that belongs to the given entity was flagged as modified or not.
    pub fn was_flagged(&self, entity: Entity) -> bool {
        self.mask.contains(entity.id())
    }
    /// Whether the component that belongs to the given entity was flagged as inserted or not.
    pub fn was_inserted(&self, entity: Entity) -> bool {
        self.inserted.contains(entity.id())
    }
    /// Whether the component that belongs to the given entity was flagged as removed or not.
    pub fn was_removed(&self, entity: Entity) -> bool {
        self.removed.contains(entity.id())
    }
    /// All components will be cleared of being flagged.
    pub fn clear_flags(&mut self) {
        self.mask.clear();
        self.inserted.clear();
        self.removed.clear();
    }
    /// Removes the modified flag for the component of the given entity.
    pub fn unflag(&mut self, entity: Entity) {
        self.mask.remove(entity.id());
    }
    /// Removes the inserted flag for the component of the given entity.
    pub fn unflag_inserted(&mut self, entity: Entity) {
        self.inserted.remove(entity.id());
    }
    /// Removes the removed flag for the component of the given entity.
    pub fn unflag_removed(&mut self, entity: Entity) {
        self.removed.remove(entity.id());
    }
    /// Flags a single component as modified.
    pub fn flag(&mut self, entity: Entity) {
        self.mask.add(entity.id());
    }
    /// Flags a single component as inserted.
    pub fn flag_inserted(&mut self, entity: Entity) {
        self.inserted.add(entity.id());
    }
    /// Flags a single component as removed.
    pub fn flag_removed(&mut self, entity: Entity) {
        self.removed.add(entity.id());
    }
}

impl<'a, C, T: UnprotectedStorage<C>> Join for &'a TrackStorage<C, T> {
    type Type = &'a C;
    type Value = &'a T;
    type Mask = &'a BitSet;
    fn open(self) -> (Self::Mask, Self::Value) {
        (&self.mask, &self.storage)
    }
    unsafe fn get(v: &mut Self::Value, id: Index) -> &'a C {
        v.get(id)
    }
}

impl<'a, C, T: UnprotectedStorage<C>> Join for &'a mut TrackStorage<C, T> {
    type Type = &'a mut C;
    type Value = &'a mut T;
    type Mask = &'a BitSet;
    fn open(self) -> (Self::Mask, Self::Value) {
        (&self.mask, &mut self.storage)
    }
    unsafe fn get(v: &mut Self::Value, id: Index) -> &'a mut C {
        // similar issue here as the `Storage<T, A, D>` implementation
        use std::mem;
        let value: &'a mut Self::Value = mem::transmute(v);
        value.get_mut(id)
    }
}

