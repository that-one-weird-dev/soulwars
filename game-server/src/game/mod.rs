use uuid::Uuid;


pub struct Game {
    pub id: Uuid,
}

impl Game {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
        }
    }
}
