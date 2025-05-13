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

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
}

impl User {
    pub fn new() -> Self {
        User { id: Uuid::new_v4() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_room() {
        let room = Room::new();
        assert!(room.users.is_empty());
    }

    #[test]
    fn test_add_user() {
        let mut room = Room::new();
        let user_id = Uuid::new_v4();
        room.add_user(user_id);
        assert!(room.users.contains(&user_id));
    }

    #[test]
    fn test_remove_user() {
        let mut room = Room::new();
        let user_id = Uuid::new_v4();
        room.add_user(user_id);
        room.remove_user(&user_id);
        assert!(!room.users.contains(&user_id));
    }

    #[test]
    fn test_create_user() {
        let user = User::new();
        assert!(user.id.to_string().len() > 0);
    }
}
