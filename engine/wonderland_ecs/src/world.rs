use crate::{Id, id::Ids};

#[derive(Debug)]
pub struct World {
    ids: Ids,
}

impl World {
    pub fn spawn(&mut self) -> Id {
        self.ids.spawn()
    }
}
