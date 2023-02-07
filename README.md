# Flea

A simple command server written in Rust.

This repository consists of three projects:

- Command Server (root)
- Simple Command Sender (flea-client)
- Flea Monitor (C#, WPF, Windows only)

The server is the main project that would be developed. The client is a test program to validate the protocol and the core functionality of the Flea Server. Flea Monitor is GUI application for a convenient managing of the server. Unless otherwise stated, the Flea works in Linux and Windows.

![Flea Monitor](https://github.com/rozensoftware/flea/blob/master/FleaMonitor.png)

The Flea Monitor would be developed simultaneously with the Flea Server. It doesn't use the client for communication with the server. It is a standalone application for Windows only.

## Purpose

- The Flea Server could be used as a spying, hacking program and/or as a remote peer for executing your special commands. The purpose can be changed according to your needs.

## Features

- Remote commands execution based on XML format
- Taking screenshots of a host
- Sending screenshot file to FTP server and/or to the client
- Uploading a file from FTP to the host
- Bash execution on a host (doesn't work perfectly on Windows)
- Key logger
- OS process list
- Killing a process
- Simple file server
- Auto update

The content of the key logger file is cleaned during the server startup.
The capabilities of the program will be increased during the development of this software.
To update the server a new version must be uploaded to the installation location. The new file name must be flea.upd. The server will be updated on the next run automatically.

## FTP Server

One of the Flea's features is the ability to send and receive files from an FTP server. The Flea Monitor provides its own FTP server, with which The Flea Server can cooperate.
You can connect to it using user name: anonymous and any password. This is an anonymous authentication. The sharing folder is created in your TEMP folder with the name 'FleaFTP'. This folder will be probably deleted on cleaning temporary folders so have it in mind.
You can start the FTP server by selecting option from File menu. There is an option for stopping it also. It works only when the Flea Monitor is running and it is shutting down on application close.

## Command format

The Flea Server requires an XML formatted command like below:

```xml
<Command name='command_name' value='optional_value'></Command>
```

Currently supported commands are:

- version : returns current server version.
- bash : a host bash command. The value property has to have a command line to execute in a host.
- ftpscreenshot : takes a screenshots of the host and sends it to FTP server.
- screenshot : takes a screenshot and sends it to the caller. The client supports this and saves data into screenshot.png file name.
- log : reads current key logger file and sends its content to receiver.
- proclist : displays currently running processes in the system.
- kill : kills a process. 'value' parameter must include PID.
- upload : uploads a file to the host from FTP server. The file name must be specified in the 'value' parameter of the XML command.
- dir : Retreives content of current directory.
- cd : Changes the current directory to the new one passed in the 'value' parameter (.. means level up).
- getfile : Downloads a file passed in the 'value' parameter to the client. The file is read from the current path on the server.
- setftp : Sets new FTP parameters: address, user name, password. Parameters must be provided in the 'value' in the following format, e.g. 127.0.0.1;user;my_pass .
- quit : Quits the program. Must be run again.

New commands will be added later.

## Flea Client

The client is used for testing, but can also be used to send commands to the Flea Server.

These are the example commands:

Get current version of the Flea Server:

```bash
./flea-client -a 127.0.0.1 -c version
```

Take screenshot and send it to the FTP server:

```bash
./flea-client -a 127.0.0.1 -c pic
```

Take screenshot and send it to the caller. It'll be saved as screenshot.png file:

```bash
./flea-client -a 127.0.0.1 -c screenshot
```

Upload test.txt file from FTP server to the host where Flea Server is running on:

```bash
./flea-client -a MY_SERVER_NAME -c upload -v test.txt
```

Change current directory to the previous one

```bash
./flea-client.exe --address MY_SERVER -c cd -v ..
```

## Building

Except having installed Rust on Linux you also need to install the following packages to build the software:

```text
build-essential, pkg-config, libx11-dev, libxcb-randr0-dev, libxcb-shm0-dev
```

On Windows you only gonna need Rust (with C++ compiler alongside).

## Installation

The Flea Server reads the host name of the computer and opens a port there. The IP address can be provided by using -s parameter. If you intend to use FTP server, you will need to complete the connection data. You can do this directly in the code:

```rust
//Enter your data for FTP Server connection
const FTP_USER_NAME: &'static str = "enter_ftp_user_name";
const FTP_PASS_NAME: &'static str = "enter_ftp_user_password";
const FTP_ADDRESS_NAME: &'static str = "enter_ftp_server_ip_address";
const FTP_FOLDER_NAME: &'static str = "enter_ftp_folder_name";

```

or in the created configuration file (recommended). On Linux you will find the configuration file in a directory: /home/user_name/.config/flea_conf/ .

Build The Flea server:

```bash
cargo build --release
```

There are various methods of running and installing this software, so I will not describe them here, as they depend directly on your preferences and needs.
The Flea program could be run on the target system, for example:

(I assume that you are typing commands in the same directory where the server executable file is located.)

```bash
./flea &
```

This run the server as a separate process.
On Windows however, you can uncomment line 1 in main.rs.

If you'd like to see debug output from the process run the command:

```bash
RUST_LOG=debug ./flea
```

You can specify IP address on which the server will be listen on:

```bash
./flea -s 127.0.0.1
```

The port number is sewn into the program code. It can be changed there or a new functionality could be written to specify it as a command parameter.
If you ran the server in the console you can stop it by CTRL-C.

## License

This project is licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contributing / Feedback

I am quite new to Rust. I am always glad to learn from anyone.
If you want to contribute, you are more than welcome to be a part of the project! Try to share you thoughts first! Feel free to open a new issue if you want to discuss new ideas.

Any kind of feedback is welcome!
