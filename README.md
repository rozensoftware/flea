# Flea
A simple command server written in Rust.

This repository consists of two projects:

- Command Server (root)
- Simple Command Sender (flea-client)

The server is the main project that would be developed. The client is a test program to validate the protocol and the core functionality of the Flea Server. Unless otherwise stated, the Flea works in Linux and Windows.

## Purpose

- The Flea Server could be used as a spying, hacking program and/or as a remote peer for executing your special commands. The purpose can be changed according to your needs.

## Features

- Remote commands execution based on XML format
- Taking screenshots of a host
- Sending screenshot file to FTP server
- Uploading a file from FTP to the host
- Bash execution on a host
- Key logger
- OS process list
- Killing a process

The content of the key logger file is cleaned during the server startup.
The capabilities of the program will be increased during the development of this software.

## Command format

The Flea Server requires an XML formatted command like below:

```xml
<Command name='command_name' value='optional_value'></Command>
```

Currently supported commands are:

* version : returns current server version
* bash : a host bash command. The value property has to have a command line to execute in a host
* pic : takes a screenshots of the host and sends it to FTP server
* sendlog : reads current key logger file and sends its content to receiver
* proclist : displays currently running processes in the system
* kill : kills a process. Value parameter must include PID
* upload : uploads a file to the host from FTP server. The file name must be specified in value parameter of the XML command

New commands will be added later.

## Flea Client

The client is used for testing, but can also be used to send commands to the Flea Server.

These are the example commands:


Get current version of the Flea Server:
```
./flea-client -a 127.0.0.1 -c version
```

Take screenshot and send it to the FTP server:
```
./flea-client -a 127.0.0.1 -c pic
```

Upload test.txt file from FTP server to the host where Flea Server is running on:
```
./flea-client -a MY_SERVER_NAME -c upload -v test.txt
```

## Building

Except having installed Rust on Linux you also need to install the following packages to build the software:

```
build-essential, pkg-config, libx11-dev, libxcb-randr0-dev, libxcb-shm0-dev
```

## Installation

The Flea Server reads the host name of the computer and opens a port there. The name can be changed to IP address with a small code modification. If you intend to use FTP server, you will need to complete the connection data. You can do this directly in the code:

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
The Flea program could be run on the target system, for example:

(I assume that you are typing commands in the same directory where the server executable file is located.)

```
./flea &
```

This run the server as a process.

If you'd like to see output from the process run the command:

```
RUST_LOG=debug ./flea
```
## License

This project is licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
at your option.

## Contributing / Feedback

I am quite new to Rust. I am always glad to learn from anyone.
If you want to contribute, you are more than welcome to be a part of the project! Try to share you thoughts first! Feel free to open a new issue if you want to discuss new ideas.

Any kind of feedback is welcome!
