
// TODO: Maybe we can make Command use &str instead of String
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Command {
    // Accounts
    NewAccount {name: String, password: String},
    DeleteAccount,
    
    // LogIn/Out
    LogIn {name: String, password: String},
    LogOut,
    
    // Room
    NewRoom(String),
    JoinRoom(String),
    LeaveRoom,

    // Messaging
    Message(String),
    PrivateMessage {user_name: String, message: String},
    
    // Friend list
    AddFriend(String),
    DeleteFriend(String),

    // Black list
    BlockUser(String),
    UnblockUser(String),

    // Query
    QueryUser(String),
    Help(String),
    
    Exit
}

impl Command {
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self)
            .expect(format!("fails to serialize {:?}", self).as_str())
    }

    pub fn deserialize(data: &[u8]) -> Command {
        bincode::deserialize(data).expect("fails to deserialize")
    }
}