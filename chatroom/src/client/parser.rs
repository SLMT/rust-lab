
use std::str::Split;

use super::super::protocol::command::Command;

// TODO: Find a better way (make it dynamic)
const HELP_MSG: &'static str = 
"Create new account\r\n\
\t[NA] [User Name] [Password]\r\n\
Delete existing account\r\n\
\t[DA]\r\n\
Log in\r\n\
\t[I] [User Name] [Password]\r\n\
Log out\r\n\
\t[O]\r\n\
Create a new room\r\n\
\t[NR] [Room Name]\r\n\
Join a room\r\n\
\t[J] [Room Name]\r\n\
Leave the room\r\n\
\t[L]\r\n\
Send message in the room\r\n\
\t[S] [Message]\r\n\
Send private message to a friend\r\n\
\t[PM] [Friend Name] [Message]\r\n\
Add a User to friendlist\r\n\
\t[NF] [User Name]\r\n\
Delete a User from friendlist\r\n\
\t[DF] [User Name]\r\n\
Add a User to blacklist\r\n\
\t[NB] [User Name]\r\n\
Delete a User from blacklist\r\n\
\t[DB] [User Name]\r\n\
Query user status\r\n\
\t[Q] [User Name]\r\n\
Help\r\n\
\t[H]\r\n\
Exit\r\n\
\t[E]\r\n";

pub fn parse(input: &str) -> Result<Command, String> {
    let mut tokens = input.split(' ');

    match tokens.next() {
        Some(token) => {
            let token = token.trim().to_uppercase();
            match token.as_str() {
                "NA" => new_account(&mut tokens),
                "DA" => Ok(Command::DeleteAccount),
                "I" => login(&mut tokens),
                "O" => Ok(Command::LogOut),
                "NR" => new_room(&mut tokens),
                "J" => join_room(&mut tokens),
                "L" => Ok(Command::LeaveRoom),
                "S" => send_message(&mut tokens),
                "PM" => private_message(&mut tokens),
                "NF" => new_friend(&mut tokens),
                "DF" => delete_friend(&mut tokens),
                "NB" => block_user(&mut tokens),
                "DB" => unblock_user(&mut tokens),
                "Q" => query_user(&mut tokens),
                "H" => Ok(Command::Help(HELP_MSG)),
                "E" => Ok(Command::Exit),
                _ => Err(format!("No such command"))
            }
        },
        None => Err(format!("No such command"))
    }
}

// TODO: use macro! to avoid duplicate code

fn new_account(tokens: &mut Split<char>) -> Result<Command, String> {
    let account = read_string_token(tokens, "account name")?;
    let password = read_string_token(tokens, "password")?;
    
    Ok(Command::NewAccount {name: account, password: password})
}

fn login(tokens: &mut Split<char>) -> Result<Command, String> {
    let account = read_string_token(tokens, "account name")?;
    let password = read_string_token(tokens, "password")?;

    Ok(Command::LogIn {name: account, password: password})
}

fn new_room(tokens: &mut Split<char>) -> Result<Command, String> {
    let room_name = read_string_token(tokens, "room name")?;

    Ok(Command::NewRoom(room_name))
}

fn join_room(tokens: &mut Split<char>) -> Result<Command, String> {
    let room_name = read_string_token(tokens, "room name")?;

    Ok(Command::JoinRoom(room_name))
}

fn send_message(tokens: &mut Split<char>) -> Result<Command, String> {
    // concat the rest of tokens as a string
    let message = tokens.collect::<Vec<&str>>().join(" ");

    Ok(Command::Message(message))
}

fn private_message(tokens: &mut Split<char>) -> Result<Command, String> {
    let user_name = read_string_token(tokens, "user name")?;
    let message = tokens.collect::<Vec<&str>>().join(" ");

    Ok(Command::PrivateMessage {user_name: user_name, message: message})
}

fn new_friend(tokens: &mut Split<char>) -> Result<Command, String> {
    let user_name = read_string_token(tokens, "user name")?;

    Ok(Command::AddFriend(user_name))
}

fn delete_friend(tokens: &mut Split<char>) -> Result<Command, String> {
    let user_name = read_string_token(tokens, "user name")?;

    Ok(Command::DeleteFriend(user_name))
}

fn block_user(tokens: &mut Split<char>) -> Result<Command, String> {
    let user_name = read_string_token(tokens, "user name")?;

    Ok(Command::BlockUser(user_name))
}

fn unblock_user(tokens: &mut Split<char>) -> Result<Command, String> {
    let user_name = read_string_token(tokens, "user name")?;

    Ok(Command::UnblockUser(user_name))
}

fn query_user(tokens: &mut Split<char>) -> Result<Command, String> {
    let user_name = read_string_token(tokens, "user name")?;

    Ok(Command::QueryUser(user_name))
}

fn read_string_token(tokens: &mut Split<char>, token_name: &str) -> Result<String, String> {
    match tokens.next() {
        Some(s) => Ok(s.to_string()),
        None => Err(format!("You missed the {}.", token_name))
    }
}

// TODO: test cases for each command (especially the message)