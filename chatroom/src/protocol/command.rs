
use std::net::TcpStream;
use std::io::{Read, Write, Result};

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

    pub fn write_to_stream<W>(&self, mut stream: W) -> Result<usize> 
        where W: Write {
        let bytes = self.serialize();
        let len_bytes = bytes.len().to_be_bytes();
        let mut byte_count: usize = 0;

        byte_count += stream.write(&len_bytes)?;
        byte_count += stream.write(&bytes)?;
        stream.flush()?;

        Ok(byte_count)
    }
    
    // TODO: Handle EOF Error
    pub fn read_from_stream<R>(mut stream: R, read_buffer: &mut Vec<u8>) -> Result<Command> 
        where R: Read {
        
        // Read the length
        let mut len_bytes: [u8; 8] = [0; 8];
        stream.read_exact(&mut len_bytes)?;
        let data_len = usize::from_be_bytes(len_bytes);

        // Ensure the size of the buffer
        let buf_len = read_buffer.len();
        if buf_len < data_len {
            let append_len = data_len - buf_len;
            let append_buf = vec![0; append_len];
            read_buffer.extend(&append_buf);
        }

        // Read the bytes
        let buf = &mut read_buffer[0 .. data_len];
        stream.read_exact(buf)?;

        Ok(Command::deserialize(buf))
    }
}