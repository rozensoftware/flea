#[cfg(target_os = "linux")]
use std::process::Command;
use log::debug;
use sysinfo::{Disks, Networks, System, Users};

#[cfg(target_os = "windows")]
use process_list::for_each_process;

#[cfg(target_os = "windows")]
use crate::windowsfunctions;

pub struct SystemCmd
{
    sys_info: System,
}

impl Default for SystemCmd
{
    fn default() -> Self
    {
        SystemCmd::new()
    }
}

impl SystemCmd
{
    pub fn new() -> SystemCmd
    {
        SystemCmd
        {
            sys_info: System::new_all(),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn execute_bash_command(&self, value: &str) -> String
    {
        use std::ffi::{c_char, CStr, CString};

        debug!("Executing bash command:{}", &value);

        //check if this is a cd command
        let v: Vec<&str> = value.split("cd ").collect();

        if v.len() > 1
        {
            return "Use 'cd' Flea command to change a directory".to_string();
        }

        extern {
            fn executeCommand(cmd: *const c_char) -> *const c_char;
        }

        let cmd = String::from(value);
        let cs = CString::new(cmd).unwrap();
        let ptr = cs.into_raw();
        let str_slice: String;

        unsafe { 
            let slice = CStr::from_ptr(executeCommand(ptr)); 
            str_slice = String::from_utf8_lossy(slice.to_bytes()).to_string();
        };

        let _cs = unsafe { CString::from_raw(ptr) };
        
        str_slice
    }

    /// Executes a command line in the current OS
    /// and returns output to the caller as a string
    /// * value - a command line to execute
    #[cfg(target_os = "linux")]
    pub fn execute_bash_command(&self, value: &str) -> String
    {
        debug!("Executing bash command:{}", &value);

        let v: Vec<&str> = value.split("cd ").collect();

        if v.len() > 1
        {
            return "Use 'cd' command to change a directory".to_string();
        }

        match Command::new(value).output()
        {
            Ok(x) =>
            {
                let str1 = format!("status: {}\n", x.status);
                let str2 = format!("stderr: {}\n", String::from_utf8_lossy(&x.stderr).to_string());
                let str3 = format!("stdout:\n{}", String::from_utf8_lossy(&x.stdout).to_string());
                
                return format!("{}{}{}", str1, str2, str3);
            },
            Err(y) =>
            {
                return y.to_string();
            }
        }        
    }

    /// Gets processes list
    /// * returns String with id and name of the processes list or empty on error
    #[cfg(target_os = "windows")]
    pub fn get_process_list(&self) -> String
    {
        let mut ret = "".to_string();

        let res = for_each_process(|id, name| {
            ret += format!("{} - {}\n", id, name.to_str().unwrap()).as_str();
        }); 

        if res.is_err()
        {
            "".to_string();
        }

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
                        if x.kill().is_ok()
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

    pub fn get_system_info(&mut self) ->String
    {
        self.sys_info.refresh_all();

        let mut ret = format!("System name: {:?}\r\nSystem kernel version: {:?}\r\nSystem OS version: {:?}\r\nSystem OS (long) version: {:?}\r\nCPUs: {}\r\n", 
            System::name().unwrap_or_else(|| "<unknown>".to_owned()),
            System::kernel_version().unwrap_or_else(|| "<unknown>".to_owned()),
            System::os_version().unwrap_or_else(|| "<unknown>".to_owned()),
            System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned()),
            self.sys_info.cpus().len());

        const MB: u64 = 1024 * 1024;

        let str = format!("Total memory: {} MB\r\nUsed memory: {} MB\r\nTotal swap: {} MB\r\nUsed swap: {} MB\r\n",
            self.sys_info.total_memory() / MB,
            self.sys_info.used_memory() / MB,
            self.sys_info.total_swap() / MB,
            self.sys_info.used_swap() / MB);

        ret.push_str(&str);
        ret.push_str("Users:\r\n");

        let users = Users::new_with_refreshed_list();

        for user in &users 
        {
            let str = format!("{:?}\r\n", user.name());
            ret.push_str(&str);
        }

        ret.push_str("Disks:\r\n");

        let disks = Disks::new_with_refreshed_list();

        for disk in &disks 
        {
            let str = format!("{:?}\r\n", disk);
            ret.push_str(&str);
        }
        
        ret.push_str("Networks:\r\n");

        let networks = Networks::new_with_refreshed_list();

        for (interface_name, data) in &networks 
        {
            let str = format!("{}: {}/{} B\r\n", interface_name, data.received(), data.transmitted());
            ret.push_str(&str);
        }

        ret.push_str("Uptime:\r\n");

        let up = System::uptime();
        let days = up / 86400;
        let hours = (up % 86400) / 3600;
        let minutes = (up % 3600) / 60;
        let seconds = up % 60;

        let str = format!("Days:{} Hours:{} Minutes:{} Seconds:{}\r\n", days, hours, minutes, seconds);
        ret.push_str(&str);

        ret
    }

    #[cfg(target_os = "windows")]
    pub fn lock_screen(&self)
    {
        extern {
            fn lockScreen();
        }
    
        unsafe {
            lockScreen();
        }
    }

    #[cfg(target_os = "linux")]
    pub fn lock_screen(&self)
    {
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