
#[derive(Debug)]
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
    Help(&'static str),
    
    Exit
}