
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
    let mut tokens = input.split(' ')
        .filter(|s| !s.is_empty()) // avoid empty token
        .map(|s| s.trim()); // remove '\n'

    match tokens.next() {
        Some(token) => {
            let token = token.to_uppercase();
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

fn new_account<'a, 'b, I>(tokens: &'a mut I) -> Result<Command, String> 
        where I: Iterator<Item = &'b str> {
    let account = read_string_token(tokens, "account name")?;
    let password = read_string_token(tokens, "password")?;
    
    Ok(Command::NewAccount {name: account, password: password})
}

fn login<'a, 'b, I>(tokens: &'a mut I) -> Result<Command, String> 
        where I: Iterator<Item = &'b str> {
    let account = read_string_token(tokens, "account name")?;
    let password = read_string_token(tokens, "password")?;

    Ok(Command::LogIn {name: account, password: password})
}

fn new_room<'a, 'b, I>(tokens: &'a mut I) -> Result<Command, String> 
        where I: Iterator<Item = &'b str> {
    let room_name = read_string_token(tokens, "room name")?;

    Ok(Command::NewRoom(room_name))
}

fn join_room<'a, 'b, I>(tokens: &'a mut I) -> Result<Command, String> 
        where I: Iterator<Item = &'b str> {
    let room_name = read_string_token(tokens, "room name")?;

    Ok(Command::JoinRoom(room_name))
}

fn send_message<'a, 'b, I>(tokens: &'a mut I) -> Result<Command, String> 
        where I: Iterator<Item = &'b str> {
    // concat the rest of tokens as a string
    let message = tokens.collect::<Vec<&str>>().join(" ");

    Ok(Command::Message(message))
}

fn private_message<'a, 'b, I>(tokens: &'a mut I) -> Result<Command, String> 
        where I: Iterator<Item = &'b str> {
    let user_name = read_string_token(tokens, "user name")?;
    let message = tokens.collect::<Vec<&str>>().join(" ");

    Ok(Command::PrivateMessage {user_name: user_name, message: message})
}

fn new_friend<'a, 'b, I>(tokens: &'a mut I) -> Result<Command, String> 
        where I: Iterator<Item = &'b str> {
    let user_name = read_string_token(tokens, "user name")?;

    Ok(Command::AddFriend(user_name))
}

fn delete_friend<'a, 'b, I>(tokens: &'a mut I) -> Result<Command, String> 
        where I: Iterator<Item = &'b str> {
    let user_name = read_string_token(tokens, "user name")?;

    Ok(Command::DeleteFriend(user_name))
}

fn block_user<'a, 'b, I>(tokens: &'a mut I) -> Result<Command, String> 
        where I: Iterator<Item = &'b str> {
    let user_name = read_string_token(tokens, "user name")?;

    Ok(Command::BlockUser(user_name))
}

fn unblock_user<'a, 'b, I>(tokens: &'a mut I) -> Result<Command, String> 
        where I: Iterator<Item = &'b str> {
    let user_name = read_string_token(tokens, "user name")?;

    Ok(Command::UnblockUser(user_name))
}

fn query_user<'a, 'b, I>(tokens: &'a mut I) -> Result<Command, String> 
        where I: Iterator<Item = &'b str> {
    let user_name = read_string_token(tokens, "user name")?;

    Ok(Command::QueryUser(user_name))
}

fn read_string_token<'a, 'b, 'c, I>(tokens: &'a mut I, token_name: &'b str) 
        -> Result<String, String>
        where I: Iterator<Item = &'c str> {
    match tokens.next() {
        Some(s) => Ok(s.to_string()),
        None => Err(format!("You missed the {}.", token_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_new_account() {
        let command = parse("NA cat meow");

        assert_eq!(command, Ok(Command::NewAccount {
            name: format!("cat"), password: format!("meow")}));
    }

    #[test]
    fn parse_string_with_consecutive_spaces() {
        let command = parse("NA cat       meow");

        assert_eq!(command, Ok(Command::NewAccount {
            name: format!("cat"), password: format!("meow")}));
    }

    #[test]
    fn parse_string_with_new_line() {
        let command = parse("NA cat meow\n");

        assert_eq!(command, Ok(Command::NewAccount {
            name: format!("cat"), password: format!("meow")}));
    }

    #[test]
    fn parse_message() {
        let command = parse("S hello! how are you?");

        assert_eq!(command, Ok(Command::Message(
            format!("hello! how are you?")
        )));
    }

    #[test]
    fn parse_private_message() {
        let command = parse("PM cat hello! how are you?");

        assert_eq!(command, Ok(Command::PrivateMessage{
            user_name: format!("cat"),
            message: format!("hello! how are you?")}
        ));
    }

    #[test]
    fn bad_command() {
        let command = parse("HAHA cat");

        assert!(command.is_err());
    }
}