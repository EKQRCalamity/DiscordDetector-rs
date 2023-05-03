# DiscordDetector-rs
Discord Advertisement/Scam/Invite Detector using Selfbots written in rust. Currently in development.

This is pretty lightweight and uses between 1-2.5 mb ram/instance and even with multiple instances 0% of CPU usage on my local test station. Supports all kinds of discord user tokens not useable with bot tokens.

You have to use the dyno bot with ? prefix.

### Currently implemented:

 - Connecting to Gateway
 - Heartbeat for keepalive
 - Deserializing Events and Messages
 - Checking for Guild and DM Messages
 - Sending of message to specific channel
 - Config file
 - Pinging of users
 - Reconnect on failed websocket connection

### ATTENTION
I dont take responsibility for anything that is done using this project or if your account gets banned. This is a **research** project.


