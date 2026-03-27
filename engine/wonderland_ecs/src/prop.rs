use std::slice::{Iter, IterMut};

use crate::Id;

pub trait Prop: Sized {
    fn get<'a>(id: Id) -> Option<&'a Self>;
    fn contains(id: Id) -> bool;
}

pub(crate) struct Pool<T: Prop> {
    sparse: Vec<Id>,
    dense: Vec<Id>,
    props: Vec<T>,
}

impl<T: Prop> Pool<T> {
    const GROWTH_FACTOR: usize = 2;

    pub fn new() -> Self {
        Self::with_universe(0.into())
    }

    pub fn with_universe(universe: Id) -> Self {
        Self {
            sparse: vec![Id::from(0); usize::from(universe)],
            dense: Vec::new(),
            props: Vec::new(),
        }
    }

    pub fn insert(&mut self, id: Id, prop: T) -> Option<T> {
        let idx = usize::from(id);
        if self.contains(id) {
            return Some(std::mem::replace(
                &mut self.props[usize::from(self.sparse[idx])],
                prop,
            ));
        }

        self.dense.push(id);
        self.props.push(prop);

        if idx >= self.sparse.len() {
            self.set_universe(Id::from(
                (idx + 1).max(self.sparse.len() * Self::GROWTH_FACTOR),
            ));
        }
        self.sparse[idx] = Id::from(self.dense.len() - 1);

        None
    }

    pub fn remove(&mut self, id: Id) -> Option<T> {
        if !self.contains(id) {
            return None;
        }

        let idx = usize::from(id);
        let dense_last_idx = self.dense.len() - 1;
        let last_idx = usize::from(self.dense[dense_last_idx]);
        let dense_idx = usize::from(self.sparse[idx]);
        self.dense.swap(dense_idx, dense_last_idx);
        self.props.swap(dense_idx, dense_last_idx);
        self.sparse.swap(idx, last_idx);

        self.dense.pop();
        self.props.pop()
    }

    pub fn contains(&self, id: Id) -> bool {
        let idx = usize::from(id);
        if idx >= self.sparse.len() {
            return false;
        }

        let dense_idx = usize::from(self.sparse[idx]);
        dense_idx < self.dense.len() && self.dense[dense_idx] == id
    }

    pub fn get(&self, id: Id) -> Option<&T> {
        self.contains(id)
            .then(|| &self.props[usize::from(self.sparse[usize::from(id)])])
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        self.contains(id)
            .then(|| &mut self.props[usize::from(self.sparse[usize::from(id)])])
    }

    pub fn universe(&self) -> Id {
        Id::from(self.sparse.len())
    }

    pub fn set_universe(&mut self, universe: Id) {
        self.sparse.resize(usize::from(universe), Id::from(0));
    }

    pub fn ids(&self) -> Iter<'_, Id> {
        self.dense.iter()
    }

    pub fn props(&self) -> Iter<'_, T> {
        self.props.iter()
    }

    pub fn props_mut(&mut self) -> IterMut<'_, T> {
        self.props.iter_mut()
    }
}

impl<T: Prop> Default for Pool<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Foo {
        id: Id,
    }

    impl Prop for Foo {
        fn get<'a>(id: Id) -> Option<&'a Foo> {
            None
        }

        fn contains(id: Id) -> bool {
            true
        }
    }

    #[test]
    fn it_inserts_prop() {
        // arrange
        let mut sut = Pool::new();
        let id = Id::from(0);
        let prop = Foo { id };

        // act
        let actual = sut.insert(id, prop);

        // assert
        assert!(actual.is_none());
        assert!(sut.contains(id));
        assert_eq!(Some(prop), sut.get(id).copied());
    }

    #[test]
    fn it_removes_prop() {
        // arrange
        let mut sut = Pool::new();
        let id = Id::from(1);
        sut.insert(Id::from(0), Foo { id: Id::from(0) });
        sut.insert(id, Foo { id });
        sut.insert(Id::from(123), Foo { id: Id::from(123) });

        // act
        let actual = sut.remove(id);

        // assert
        assert_eq!(Some(Foo { id }), actual);
        assert!(!sut.contains(id));
        assert!(sut.contains(Id::from(0)));
        assert!(sut.contains(Id::from(123)));
    }
}
