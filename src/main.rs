use ssh2::*;
use std::io::{Read, stdin};
use std::net::TcpStream;


fn input_username() -> String {
    let mut input_username = String::new();
    println!("Please input your username:");
    stdin().read_line(&mut input_username).unwrap();
    input_username.trim().to_string()
}

fn input_password() -> String {
    let mut input_password = String::new();
    println!("Please input your password:");
    stdin().read_line(&mut input_password).unwrap();
    input_password.trim().to_string()
}

/*fn read_user_list() -> Vec<String> {
    let mut user_list = Vec::new();
    let mut input_user_list = String::new();
    stdin().read_line(&mut input_user_list).unwrap();
    for user in input_user_list.split_whitespace() {
        user_list.push(user.to_string());
    }
    user_list
}*/

/*fn read_password_list() -> Vec<String> {
    let mut password_list = Vec::new();
    let mut input_password_list = String::new();
    stdin().read_line(&mut input_password_list).unwrap();
    for password in input_password_list.split_whitespace() {
        password_list.push(password.to_string());
    }
    password_list
}*/

fn read_ip_address() -> String {
    let mut input_ip_address = String::new();
    println!("Please input the ip address of the server:");
    stdin().read_line(&mut input_ip_address).unwrap();
    input_ip_address.trim().to_string()
}

fn read_port() -> u16 {
    let mut input_port = String::new();
    println!("Please input the port of the server:");
    stdin().read_line(&mut input_port).unwrap();
    input_port.trim().parse::<u16>().unwrap()
}

fn main() {
    let ip_address = read_ip_address();
    let port = read_port();
    let username = input_username();
    let password = input_password();

    let mut session = Session::new().unwrap();
    let tcp = TcpStream::connect(&format!("{}:{}", ip_address, port)).unwrap();
    println!("Connecting to {}:{}", ip_address, port);

    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    session.userauth_password(&username, &password).unwrap();

    assert!(session.authenticated());
    println!("Connected with user {}", username);

    let mut channel = session.channel_session().unwrap();
    channel.exec("ls -la").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());
}

