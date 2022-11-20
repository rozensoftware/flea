# flea
A simple command server written in Rust.

This repository consists of two projects:

- Command Server (root)
- Simple Command Sender (flea-client)

The server is the main project that would be developed. The client is a test program to validate the protocol and the core functionality of the Flea Server.

## Purpose

- The Flea Server could be used as a spying, hacking program and/or as a remote peer for executing your special commands. The purpose can be changed according to your needs.

## Features

- Remote commands execution based on XML format
- Taking screenshots of a host
- Sending screenshot file to FTP server
- Bash execution on a host

The capabilities of the program will be increased during the development of this software.

## Installation

Depending on your application, you will have to change the server's IP address and its port in the code (../flea/src/main.rs) which it will be listen on. If you intend to use FTP server, you will need to complete the connection data. You can do this directly in the code:

```rust
//Enter your data for FTP Server connection
const FTP_USER_NAME: &'static str = "enter_ftp_user_name";
const FTP_PASS_NAME: &'static str = "enter_ftp_user_password";
const FTP_ADDRESS_NAME: &'static str = "enter_ftp_server_ip_address";
const FTP_FOLDER_NAME: &'static str = "enter_ftp_folder_name";

```

or in the created configuration file (recommended). On Linux you will find the configuration file in a directory: /home/user_name/.config/flea_conf/ .


Build The Flea server:

```
cargo build --release
```

There are various methods of running and installing this software, so I will not describe them here, as they depend directly on your preferences and needs.
The flea program could be run on the target system, for example:

(I assume that you are typing commands in the same directory where the server executable file is located.)

```
./flea &
```

This run the server as a process.

If you'd like to see output from the process run the command:

```
RUST_LOG=debug ./flea
```

## Contributing / Feedback

I am quite new to Rust. I am always glad to learn from anyone.
If you want to contribute, you are more than welcome to be a part of the project! Try to share you thoughts first! Feel free to open a new issue if you want to discuss new ideas.

Any kind of feedback is welcome!