use std::env;

use crate::gateway::getgateway;
mod gateway;
mod config;
mod message;
use crate::config::{getconfig, ConfigStruct, configconstructor};

pub fn read_input(prompt: &str) -> String {
    use std::io::{self, Write};
    let mut buffer: String = String::new();
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_owned()
}

fn main() {
    let token: &str;
    let args: Vec<_> = env::args().collect();
    let possibleuserinput: String;
    let strinput: String;
    let mut config: ConfigStruct;
    if args.len() > 1 {
        if args[1] == "-c" {
            config = configconstructor();
            if args.len() > 2 {
                token = args[2].as_str();
            } else {
                config = getconfig();
                possibleuserinput = read_input("Please enter the user token:");
                strinput = possibleuserinput.as_str().to_owned();
                token = &strinput;        
            }
        } else {
            config = getconfig();
            token = args[1].as_str();
        }
    } else {
        config = getconfig();
        possibleuserinput = read_input("Please enter the user token:");
        strinput = possibleuserinput.as_str().to_owned();
        token = &strinput;
    }

    let url = getgateway(gateway::DiscordAPIVersions::V10);
    let actualurl = url + "/?v=10&encoding=json";
    println!("Connecting to: {}", actualurl);
    gateway::connect_to(&actualurl.as_str(), token, config.channelid, &config.ping);
}


