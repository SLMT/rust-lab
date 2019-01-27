
use std::collections::HashSet;
use std::collections::hash_set::Iter;

use super::IdType;

pub struct Room {
    id: IdType,
    
    name: String,
    members: HashSet<IdType>
}

impl Room {
    fn new(id: IdType, name: String) -> Room {
        Room {
            id: id,
            name: name,
            members: HashSet::new()
        }
    }

    fn add_member(&mut self, user_id: IdType) -> bool {
        self.members.insert(user_id)
    }

    fn remove_member(&mut self, user_id: IdType) -> bool {
        self.members.remove(&user_id)
    }

    fn contain_member(&self, user_id: IdType) -> bool {
        self.members.contains(&user_id)
    }

    fn get_member_iterator(&self) -> Iter<IdType> {
        self.members.iter()
    }
}