use crate::Prop;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Id(usize);

impl Id {
    const POS_BITS: usize = 20;
    const POS_MAX: usize = (1 << Self::POS_BITS) - 1;
    const POS_MASK: usize = Self::POS_MAX;

    const VER_BITS: usize = 12;
    const VER_MAX: usize = (1 << Self::VER_BITS) - 1;
    const VER_MASK: usize = Self::VER_MAX << Self::POS_BITS;

    pub(crate) const MAX: usize = Self::POS_MAX | (Self::VER_MAX << Self::POS_BITS);

    const fn new(pos: usize, ver: usize) -> Self {
        Self(pos | (ver << Self::POS_BITS))
    }

    const fn pos(&self) -> usize {
        self.0 & Self::POS_MASK
    }

    const fn ver(&self) -> usize {
        (self.0 & Self::VER_MASK) >> Self::POS_BITS
    }

    pub fn get<'a, T: Prop>(&self) -> Option<&'a T> {
        T::get(*self)
    }

    pub fn has<T: Prop>(&self) -> bool {
        T::contains(*self)
    }
}

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        assert!(value <= Id::MAX);
        Self(value)
    }
}

impl From<Id> for usize {
    fn from(value: Id) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub(crate) struct Ids {
    inner: Vec<Id>,
    available_cnt: usize,
    next_pos: usize,
}

impl Ids {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            available_cnt: 0,
            next_pos: 0,
        }
    }

    pub fn spawn(&mut self) -> Id {
        if self.available_cnt > 0 {
            self.recycle()
        } else {
            self.spawn_impl()
        }
    }

    pub fn despawn(&mut self, id: Id) {
        if self.is_dead(id) {
            return;
        }

        let pos_to_despawn = id.pos();
        let next_ver = if id.ver() + 1 > Id::VER_MAX {
            0
        } else {
            id.ver() + 1
        };
        self.inner[pos_to_despawn] = Id::new(self.next_pos, next_ver);

        self.next_pos = pos_to_despawn;
        self.available_cnt += 1;
    }

    pub fn is_alive(&self, id: Id) -> bool {
        let id_pos = id.pos();
        let id_ver = id.ver();

        id_pos < self.inner.len() && id_ver == self.inner[id_pos].ver()
    }

    pub fn is_dead(&self, id: Id) -> bool {
        !self.is_alive(id)
    }

    fn recycle(&mut self) -> Id {
        let pos_to_recycle = self.next_pos;
        // holder stores recycled Id version
        let holder = self.inner[pos_to_recycle];

        let recycled = Id::new(pos_to_recycle, holder.ver());

        self.next_pos = holder.pos();
        self.available_cnt -= 1;

        recycled
    }

    fn spawn_impl(&mut self) -> Id {
        assert!(self.next_pos <= Id::POS_MAX, "ids limit exceeded");

        // freshly spawned Id has version equals to 0
        let spawned = Id(self.next_pos);
        self.inner.push(spawned);

        self.next_pos += 1;

        spawned
    }
}

impl Default for Ids {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_spawns_id() {
        // arrange
        let mut sut = Ids::new();
        let expected = Id(0);

        // act
        let actual = sut.spawn();

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_despawns_id() {
        // arrange
        let mut sut = Ids::new();
        let alive = sut.spawn();
        let dead = sut.spawn();

        // act
        sut.despawn(dead);

        // assert
        assert!(sut.is_alive(alive));
        assert!(sut.is_dead(dead));
    }

    #[test]
    fn it_recycles_id() {
        // arrange
        let mut sut = Ids::new();

        // act
        for _ in 0..10 {
            sut.spawn();
        }

        for i in (0..10).step_by(2) {
            sut.despawn(Id(i))
        }

        // assert
        for i in 0..10 {
            if (i & 1) == 0 {
                assert!(sut.is_dead(Id(i)))
            } else {
                assert!(sut.is_alive(Id(i)))
            }
        }

        for i in (0..10).step_by(2) {
            let expected = Id((8 - i) | (1 << 20));
            let actual = sut.spawn();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn stress_test_recycling_capability() {
        // arrange
        let mut sut = Ids::new();
        let expected = Id(0);
        let next_expected = Id(1);

        // act
        for _ in 0..4096 {
            let spawned = sut.spawn();
            sut.despawn(spawned);
        }
        let actual = sut.spawn();
        let next_actual = sut.spawn();

        // assert
        assert_eq!(expected, actual);
        assert_eq!(next_expected, next_actual);
    }

    #[test]
    fn it_despawns_id_idempotently() {
        // arrange
        let mut sut = Ids::new();
        let target = sut.spawn();
        sut.despawn(target);

        // Only one increment of the version expected
        let expected_recycled = Id(1 << 20);
        let expected_spawned = Id(1);

        // act
        sut.despawn(target);
        let recycled = sut.spawn();
        let spawned = sut.spawn();

        // assert
        assert_eq!(expected_recycled, recycled);
        assert_eq!(expected_spawned, spawned);
    }
}
