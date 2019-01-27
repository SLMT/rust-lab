
use std::collections::HashSet;
use std::collections::hash_set::Iter;

use super::IdType;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum UserStatus {
    OFFLINE,
    LOBBY,
    IN_ROOM(IdType) // including room id
}

pub struct User {
    id: IdType,
    
    name: String,
    password: String,

    current_status: UserStatus,
    
    friend_list: HashSet<IdType>,
    black_list: HashSet<IdType>
}

impl User {
    fn new(id: IdType, name: String, password: String) -> User {
        User {
            id: id,
            name: name,
            password: password,
            current_status: UserStatus::OFFLINE,
            friend_list: HashSet::new(),
            black_list: HashSet::new()
        }
    }

    fn get_id(&self) -> IdType {
        self.id
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn check_password(&self, password: String) -> bool {
        self.password == password
    }

    fn set_status(&mut self, new_status: UserStatus) {
        self.current_status = new_status;
    }

    fn get_status(&self) -> UserStatus {
        self.current_status
    }

    fn add_friend(&mut self, friend_id: IdType) -> bool {
        self.friend_list.insert(friend_id)
    }

    fn remove_friend(&mut self, friend_id: IdType) -> bool {
        self.friend_list.remove(&friend_id)
    }

    fn contain_friend(&self, friend_id: IdType) -> bool {
        self.friend_list.contains(&friend_id)
    }

    fn get_friend_iterator(&self) -> Iter<IdType> {
        self.friend_list.iter()
    }

    fn block_user(&mut self, blocked_id: IdType) -> bool {
        self.black_list.insert(blocked_id)
    }

    fn unblock_user(&mut self, blocked_id: IdType) -> bool {
        self.black_list.remove(&blocked_id)
    }

    fn in_black_list(&self, user_id: IdType) -> bool {
        self.black_list.contains(&user_id)
    }

    fn get_black_list_iterator(&self) -> Iter<IdType> {
        self.black_list.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_creation() {
        let user = User::new(1, format!("{}", "name"), format!("{}", "password"));

        assert_eq!(1, user.get_id());
        assert_eq!("name", user.get_name());
        assert!(user.check_password(format!("{}", "password")));
        assert_eq!(UserStatus::OFFLINE, user.get_status());
    }
}