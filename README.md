# rusty-mc-server

A little experiment in implementing a bit of the Minecraft protocol in Rust.

Not under active development, though you probably already know that (just look at the commit history :wink:)

I might revisit the project at some point in the future, who knows ¯\\\_(ツ)_/¯

## What Works

Err, almost nothing!

Pinging the server works, and you're able to make it thorough some of the login flow, but that's about it!

## What's the Plan?

This project is mainly about messing around with network programming a bit, and seeing if I can implement enough of the minecraft server protocol to get a player in-game, and send them some chunk data.

The end goal isn't to make a replacement for the standard Minecraft server.jar. That would take waaaay too long, and isn't something I even have interest in doing. See [feather](https://github.com/caelunshun/feather) if you're interested in a proper Minecraft server written in Rust.
