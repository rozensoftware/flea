use std::process::Stdio;

use execute::{Execute, command};
use log::debug;

#[cfg(target_os = "windows")]
use process_list::for_each_process;

#[cfg(target_os = "windows")]
use crate::windowsfunctions;

pub struct SystemCmd
{
}

impl SystemCmd
{
    pub fn new() -> SystemCmd
    {
        SystemCmd
        {
        }
    }

    /// Executes a command line in the current OS
    /// and returns output to the caller as a string
    /// * value - a command line to execute
    /// TODO: On Windows it doesn't work correctly
    pub fn execute_bash_command(&self, value: &str) -> String
    {
        debug!("Executing bash command:{}", &value);

        let mut command = command(value);

        command.stdout(Stdio::piped());
        
        let output = match command.execute_output()
        {
            Ok(x) =>
            {
                x
            },
            Err(y) =>
            {
                return y.to_string()
            }
        };
        
        String::from_utf8(output.stdout).unwrap()
    }

    /// Gets processes list
    /// * returns String with id and name of the processes list or empty on error
    #[cfg(target_os = "windows")]
    pub fn get_process_list(&self) -> String
    {
        let mut ret = "".to_string();

        match for_each_process(|id, name| {
            ret += format!("{} - {}\n", id, name.to_str().unwrap().to_string()).as_str();
        })
        {
            Ok(_) =>
            {
            },
            Err(_) =>
            {
            }
        };

        ret
    }

    #[cfg(target_os = "windows")]
    pub fn kill_process(&self, pid: &str) -> String
    {
        use std::str::FromStr;

        use log::error;

        debug!("Executing kill process..");

        match u32::from_str(pid)
        {
            Ok(num) =>
            {
                match windowsfunctions::WindowsProcess::open(num)
                {
                    Ok(x) =>
                    {
                        if let Ok(_) = x.kill()
                        {
                            debug!("Process killed");
                            return "Ok".to_string();
                        };
                    },
                    Err(y) =>
                    {
                        error!("{}", y);
                        return y;
                    }
                }                    
            },
            Err(e) =>
            {
                return e.to_string();
            }
        }

        error!("Process {} couldn't be killed.", pid);

        "Couldn't kill the process".to_string()
    }

    /// Gets processes list (Linux version)
    /// * returns String with id and name of the processes list or empty on error
    #[cfg(target_os = "linux")]
    pub fn get_process_list(&self) -> String
    {
        self.execute_bash_command("ps aux")
    }

    ///Kill process
    /// * pid - PID of the process
    #[cfg(target_os = "linux")]
    pub fn kill_process(&self, pid: &str) -> String
    {
        let s = format!("kill {}", pid);
        format!("Bash executed {}", self.execute_bash_command(&s))
    }
}