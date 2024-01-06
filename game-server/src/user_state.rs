use std::sync::RwLock;

use uuid::Uuid;


#[derive(Default)]
pub struct UserState {
    id: RwLock<Uuid>,
}

impl UserState {
    pub fn init(&self, id: Uuid) {
        *self.id.write().unwrap() = id;
    }

    pub fn id(&self) -> Uuid {
        return *self.id.read().unwrap();
    }
}

