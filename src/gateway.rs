use std::time::Duration;
use std::thread;

use serde::{Deserialize, Serialize};
use serde_json::Error;
use tungstenite::stream::MaybeTlsStream;
use serde_json::{Map, Value};
use tungstenite::{connect, Message};
use url::Url;
use chrono::{Datelike, Timelike, Local};

use crate::message;
use crate::message::{sendmessage, MessageObj};

#[derive(Serialize, Deserialize)]
struct GatewayResponse {
    url: String
}

#[derive(Serialize, Deserialize)]
struct HeartbeatResponse {
    t: Option<String>,
    s: Option<String>,
    op: i32,
    d: Map<String, Value>
}

#[derive(Serialize, Deserialize)]
struct HeartbeatData {
    heartbeat_interval: u128
}


impl HeartbeatData {
    fn new(heartbeat_interval: u128) -> HeartbeatData {
        HeartbeatData { heartbeat_interval: heartbeat_interval }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageRoot {
    pub t: String,
    pub s: i64,
    pub op: i64,
    pub d: MessageStruct,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuildMessageRoot {
    pub t: String,
    pub s: i64,
    pub op: i64,
    pub d: GuildMessageStruct,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsualRoot {
    pub t: String,
    pub s: i64,
    pub op: i64,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailsafeRoot {
    pub op: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageStruct {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub tts: bool,
    pub timestamp: String,
    pub referenced_message: Value,
    pub pinned: bool,
    pub nonce: String,
    pub mentions: Vec<Value>,
    pub mention_roles: Vec<Value>,
    pub mention_everyone: bool,
    pub id: String,
    pub flags: i64,
    pub embeds: Vec<Value>,
    pub edited_timestamp: Value,
    pub content: String,
    pub components: Vec<Value>,
    pub channel_id: String,
    pub author: Author,
    pub attachments: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuildMessageStruct {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub tts: bool,
    pub timestamp: String,
    pub referenced_message: Value,
    pub pinned: bool,
    pub nonce: String,
    pub mentions: Vec<Value>,
    pub mention_roles: Vec<Value>,
    pub mention_everyone: bool,
    pub member: Member,
    pub id: String,
    pub flags: i64,
    pub embeds: Vec<Value>,
    pub edited_timestamp: Value,
    pub content: String,
    pub components: Vec<Value>,
    pub channel_id: String,
    pub author: Author,
    pub attachments: Vec<Value>,
    pub guild_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub username: String,
    pub public_flags: i64,
    pub id: String,
    pub global_name: Value,
    pub display_name: Value,
    pub discriminator: String,
    pub avatar_decoration: Value,
    pub avatar: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub roles: Vec<String>,
    pub premium_since: Value,
    pub pending: bool,
    pub nick: Value,
    pub mute: bool,
    pub joined_at: String,
    pub flags: i64,
    pub deaf: bool,
    pub communication_disabled_until: Value,
    pub avatar: Value,
}

pub fn newvecfromvec(vec: &Vec<u64>) -> Vec<u64> {
    let newvec = vec.to_vec();
    return newvec;
}

pub fn connect_to(url: &str, token: &str, channelid: u64, pings: &Vec<u64>) {
    let resc = connect(
        Url::parse(url).unwrap()
    );

    match &resc {
        Ok(_res) => (),
        Err(_err) => {
            println!("Couldn't connect to {}\nRetrying in 5 seconds", url);
            thread::sleep(Duration::from_secs(5));
            connect_to(url, token, channelid, pings);
        }
    }

    let (mut socket, _response) = resc.unwrap(); // Unwrap Socket from Result<>
    let msg = socket.read_message().expect("Error reading message");

    let re: HeartbeatResponse = serde_json::from_str(msg.to_text().unwrap()).unwrap();
    
    let hbintervalasu128: u128 = u128::try_from(re.d.get("heartbeat_interval").unwrap().as_u64().unwrap()).expect("Heartbeat could not be converted to u128"); 
    
    let hbdata: HeartbeatData = HeartbeatData::new(hbintervalasu128);

    let jsondata = format!(r#"{{"op": 1, "d": "null"}}"#);
    let payload = Message::Text(jsondata);
    socket.write_message(payload).expect("Error sending heartbeat");
    socket.read_message().expect("Error reading heartbeat!");

    println!("[{:02}.{:02}.{:02}] - First Heartbeat sent at {:02}:{:02}:{:02}!", Local::now().day(), Local::now().month(), Local::now().year(), Local::now().hour(), Local::now().minute(), Local::now().second());
    
    let pingvec = newvecfromvec(&pings);
    
    match socket.get_mut() {
        MaybeTlsStream::NativeTls(t) => {
            t.get_mut().set_read_timeout(Some(std::time::Duration::from_millis((hbdata.heartbeat_interval as f64 / 1.2) as u64))).expect("Error setting read timeout");
        },

        _ => panic!("Unsupported stream!"),
    }
    
    let authdata: String = format!(r#"{{"op": 2,"d": {{"token": "{}","properties": {{"os": "linux","browser": "firefox","device": "pc"}}}}}}"#, token);
    let authpayload = Message::Text(authdata);
    socket.write_message(authpayload).expect("Failed to send auth payload");
    socket.read_message().expect("Failed reading auth response!");

    let pingclone = pingvec.to_vec();
    let mut lasts = 0;
    
    loop {
        let msg2 = match socket.read_message() {
            Ok(msg) => msg,
            Err(_err) => Message::Text(format!("")),
        };

        if !msg2.is_empty() {
            
            if msg2.to_string().contains("\"t\":\"MESSAGE_CREATE\",") {
                let msgrecvobj: Result<GuildMessageRoot, Error> = serde_json::from_str(msg2.to_text().unwrap());
                match msgrecvobj {
                    Ok(msgobj) => {
                        println!("[{:02}.{:02}.{:02}] - Was a guild message - ignored", Local::now().day(), Local::now().month(), Local::now().year());
                    },
                    Err(_err) => {
                        let personalmessageobj: Result<MessageRoot, Error> = serde_json::from_str(msg2.to_text().unwrap());
                        match personalmessageobj {
                            Ok(msgobj) => {
                                let mut pingstr = "".to_string();
                                for ping in &pingclone {
                                    pingstr += format!("<@{}> ", ping.to_string()).as_str();
                                }
                                let msg1: MessageObj = message::MessageObj::new(format!("{}\nMessage from <@{}> - {}#{} received:\n\n{}", pingstr, msgobj.d.author.id, msgobj.d.author.username, msgobj.d.author.discriminator, msgobj.d.content));
                                sendmessage(msg1, token, channelid);
                                sendmessage(message::MessageObj::new(format!("?ban <@{}> 0 \"Sent a dm! Likely spam\"", msgobj.d.author.id)), token, channelid);
                                lasts = msgobj.s;
                            },
                            Err(_err) => {
                                let usualobj: Result<UsualRoot, Error> = serde_json::from_str(msg2.to_text().unwrap());
                                match usualobj {
                                    Ok(obj) => {
                                        println!("[{:02}.{:02}.{:02}] - Weird message received - OP Code: {}, S: {}, Type: {}", Local::now().day(), Local::now().month(), Local::now().year(), obj.op, obj.s, obj.t);
                                        lasts = obj.s;
                                    },
                                    Err(_err) => {
                                        let ignoretypeobject: Result<FailsafeRoot, serde_json::Error> = serde_json::from_str(msg2.to_text().unwrap());
                                        match ignoretypeobject {
                                            Ok(msg) => println!("[{:02}.{:02}.{:02}] - Failsafe Message received: OP Code = {}", Local::now().day(), Local::now().month(), Local::now().year(), msg.op),
                                            Err(_err) => {
                                                println!("[{:02}.{:02}.{:02}] - Error parsing response... {}", Local::now().day(), Local::now().month(), Local::now().year(), _err);
                                            },
                                        }
                                    }
                                }
                            },
                        }
                    },
                }
            } else {
                let usualobj: Result<UsualRoot, Error> = serde_json::from_str(msg2.to_text().unwrap());
                match usualobj {
                    Ok(obj) => {
                        println!("[{:02}.{:02}.{:02}] - Usual Message received: OP Code = {}, S = {}, Type = {}", Local::now().day(), Local::now().month(), Local::now().year(), obj.op, obj.s, obj.t);
                        lasts = obj.s;

                    },
                    Err(_err) => {
                        let ignoretypeobject: Result<FailsafeRoot, serde_json::Error> = serde_json::from_str(msg2.to_text().unwrap());
                        match ignoretypeobject {
                            Ok(msg) => println!("[{:02}.{:02}.{:02}] - Failsafe Message received: OP Code = {}", Local::now().day(), Local::now().month(), Local::now().year(), msg.op),
                            Err(_err) => {
                                println!("[{:02}.{:02}.{:02}] - Error parsing response... {}", Local::now().day(), Local::now().month(), Local::now().year(), _err);
                            },
                        }
                        
                    }
                }
            }
        }
        
        let jsondata = format!(r#"{{"op": 1, "d": {}}}"#, lasts);
        let payload = Message::Text(jsondata);
        let respo = socket.write_message(payload);
        
        match respo {
            Ok(_msg) => (),
            Err(_err) => {
                connect_to(url, token, channelid, &pingvec);
                break;
            },
        }
        
        match socket.read_message() {
            Ok(_msg) => (),
            Err(_err) => {
                connect_to(url, token, channelid, &pingvec);
                break;
            },
        }
        
        println!("[{:02}.{:02}.{:02}] - Heartbeat sent at {:02}:{:02}:{:02}!", Local::now().day(), Local::now().month(), Local::now().year(), Local::now().hour(), Local::now().minute(), Local::now().second());
    }
}

#[derive(PartialEq)]
pub enum DiscordAPIVersions {
    V9,
    V10
}

fn fetchgateway(v: DiscordAPIVersions) -> String {
    let response = reqwest::blocking::get(format!("https://discord.com/api/{}/gateway", if v == DiscordAPIVersions::V9 { "v9" } else { "v10" })).unwrap();
    let _status = response.status();
    let body = response.text().unwrap();
    let re: GatewayResponse = serde_json::from_str(body.as_str()).unwrap();
    re.url
}

pub fn getgateway(v: DiscordAPIVersions) -> String {
    let gateway: String = fetchgateway(v);
    gateway
}

