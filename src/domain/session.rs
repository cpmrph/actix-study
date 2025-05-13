use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug)]
pub struct Room {
    pub id: Uuid,
    pub users: HashSet<Uuid>,
}

impl Room {
    pub fn new() -> Self {
        Room {
            id: Uuid::new_v4(),
            users: HashSet::new(),
        }
    }

    pub fn add_user(&mut self, user_id: Uuid) {
        self.users.insert(user_id);
    }

    pub fn remove_user(&mut self, user_id: &Uuid) {
        self.users.remove(user_id);
    }
}

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
}

impl User {
    pub fn new() -> Self {
        User { id: Uuid::new_v4() }
    }
}
