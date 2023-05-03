use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use serde::{Serialize};
use reqwest::blocking::Client;
use reqwest::header;

#[derive(Serialize)]
pub struct MessageObj {
    content: String,
    flags: u64,
    nonce: String,
    tts: bool,
}

pub fn noncegenerator() -> String {
    let mut rng = thread_rng();
    let range = Uniform::new_inclusive(10u64.pow(18), 10u64.pow(19) - 1);
    let num: u64 = rng.sample(range);
    return num.to_string();
}

impl MessageObj {
    pub fn new(content: String) -> MessageObj {
        MessageObj { content: content, flags: 0, nonce: noncegenerator(), tts: false }
    }
}

pub fn sendmessage(msgobj: MessageObj, token: &str, channel_id: u64) {
    let reqclient = Client::new();
    let mut serstring = String::new();
    let serialized = serde_json::to_string(&msgobj);
    match serialized {
        Ok(message) => {
            //println!("{}", message);
            serstring = message;
        },
        Err(_err) => panic!("Error serializing message!"),
    }
    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(token).unwrap());
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) discord/1.0.9012 Chrome/108.0.5359.215 Electron/22.3.2 Safari/537.36"));
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
    let apiurl = format!("https://discord.com/api/v9/channels/{}/messages", channel_id);
    let response = reqclient.post(apiurl)
                                    .body(serstring)
                                    .headers(headers)
                                    .send();
    match response {
        Ok(_response) => print!(""),
        Err(_err) => panic!("Error occured while sending message: {}", _err),
    }
}
