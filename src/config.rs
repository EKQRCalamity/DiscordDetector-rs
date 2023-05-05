use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use std::thread;

pub fn read_input(prompt: &str) -> String {
    use std::io::{self};
    let mut buffer: String = String::new();
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_owned()
}

#[derive(Serialize, Deserialize)]
pub struct ConfigStruct {
    pub channelid: u64,
    pub ping: Vec<u64>,
}

pub fn configconstructor() -> ConfigStruct {
    println!("Config setup entered!");
    let useridinput = read_input("User(s) to ping (Seperate IDs with space):");
    let ids = useridinput.split(" ");
    let mut idstring = "".to_string();
    for id in ids {
        idstring += format!("{}, ", id).as_str();
    }
    idstring.truncate(idstring.len() - 2);
    let input = read_input("Enter channel ID:");
    let jsonstring = format!(r#"{{"channelid": {}, "ping": [{}]}}"#, input.parse::<u64>().unwrap(), idstring);
    saveconfig(jsonstring);
    thread::sleep(Duration::from_secs(1));
    return getconfig();
}

pub fn saveconfig(configstr: String) {
    let bytestr = configstr.as_bytes();
    let mut cwriter = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .open("disdetect.config")
                                .unwrap();
    cwriter.write_all(bytestr).expect("Unable to write config!");
}

pub fn hasconfig() -> bool {
    return Path::new("disdetect.config").exists();
}

pub fn getconfig() -> ConfigStruct {
    if hasconfig() {
        let data = fs::read_to_string("disdetect.config").expect("Failed to read config file!");
        let config: ConfigStruct = serde_json::from_str(data.as_str()).expect("Unable to parse config file!");
        return config;
    } else {
        return configconstructor();
    }
}