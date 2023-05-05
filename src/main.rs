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

struct ArgPasser {
    token: &str,
    config: ConfigStruct,
}

impl ArgPasser {
    fn new(t: &str, c: ConfigStruct) {
        ArgPasser { token: t, config: c }
    }

    fn setConfig(c: ConfigStruct) {
        self.config = c;
    }    

    fn setToken(t: &str) {
        self.token = t;
    }
}

fn handleargs(args: &Vec<_>) -> ArgPasser {
    passer: ArgPasser = ArgPasser::new("", getconfig());
    if args.len() > 1 {
        for argument in args {
            if argument == "-c" {
                config = configconstructor
                passer.setConfig(config);
            } else {
                passer.setToken(argument.as_str());
            }
        }
    } else {
        config = getconfig();
        possibleuserinput = read_input("Please enter the user token:");
        strinput = possibleuserinput.as_str().to_owned();
        token = &strinput;  
    }
    return passer;
}

fn main() {
    let token: &str;
    let args: Vec<_> = env::args().collect();
    let possibleuserinput: String;
    let strinput: String;
    let mut config: ConfigStruct;
    argpasser = handleargs(&args);
    let url = getgateway(gateway::DiscordAPIVersions::V10);
    let actualurl = url + "/?v=10&encoding=json";
    println!("Connecting to: {}", actualurl);
    gateway::connect_to(&actualurl.as_str(), argpasser.token, argpasser.config.channelid, &argpasser.config.ping);
}


