# **This application is currently NOT functioning, I will have to update it for the discord name change and if I update it I can just rewrite it in a better way. Source will follow as soon as I finished rewriting it.**

# DiscordDetector-rs
Discord Advertisement/Scam/Invite Detector using Selfbots written in rust. Currently in development.

This is pretty lightweight and uses between 1-2.5 mb ram/instance and even with multiple instances 0% of CPU usage on my local test station. Supports all kinds of discord user tokens not useable with bot tokens.



### Currently implemented:

 - Connecting to Gateway
 - Heartbeat for keepalive
 - Deserializing Events and Messages
 - Checking for Guild and DM Messages
 - Sending of message to specific channel
 - Config file
 - Pinging of users
 - Reconnect on failed websocket connection

### Usage:

 - Setup Dyno for normal users to have ban rights and add them to a private channel.
 - You have to use the dyno bot with ? prefix.
 - dd-detector [-c | token]
 - With -c you will enter the config setup even if the config file is already created without it provided it will look for the config file and create it with inital config setup if there is no config file.
 - The token can be provided as argument or typed in after

### Building:

 - Download Cargo (Not just rustc).
 - Clone this repo `git clone https://github.com/EKQRCalamity/DiscordDetector-rs.git`
 - Change the directory to the root directory of the repo `cd DiscordDetector-rs`
 - Build the project `cargo build` or `cargo build --release`

### ATTENTION
I dont take responsibility for anything that is done using this project or if your account gets banned. This is a **research** project.

**THIS WONT BE ABLE TO BE USED ON SMALL SERVERS WHERE USERS MIGHT ACTUALLY DM EACHOTHER. RECOMMEND TO HAVE A SERVER WITH AT LEAST A COUPLE HUNDRED USERS**

