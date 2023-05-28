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
    token: String,
    config: ConfigStruct,
}

impl ArgPasser {
    pub fn new(t: String, c: ConfigStruct) -> ArgPasser {
        return ArgPasser { token: t, config: c };
    }

    fn setconfig(&mut self, c: ConfigStruct) {
        self.config = c;
    }    

    fn settoken(&mut self, t: String) {
        self.token = t;
    }
}

fn handleargs(args: &mut[String]) -> ArgPasser {
    let mut passer: ArgPasser = ArgPasser::new("".to_string(), getconfig());
    if args.len() > 1 {
        for argument in args {
            if argument == "-c" {
                let config = configconstructor();
                passer.setconfig(config);
            } else {
                passer.settoken(argument.to_string());
            }
        }
    } else {
        let config = getconfig();
        let possibleuserinput = read_input("Please enter the user token:");
        let strinput = possibleuserinput.as_str().to_owned();
        let token = &strinput;  
        passer.setconfig(config);
        passer.settoken(token.to_string());
    }
    return passer;
}

fn main() {
    let mut args: Vec<_> = env::args().collect();
    let argpasser = handleargs(args.as_mut_slice());
    let url = getgateway(gateway::DiscordAPIVersions::V10);
    let actualurl = url + "/?v=10&encoding=json";
    println!("Connecting to: {}", actualurl);
    gateway::connect_to(&actualurl.as_str(), argpasser.token.to_string().as_str(), argpasser.config.channelid, &argpasser.config.ping);
}


