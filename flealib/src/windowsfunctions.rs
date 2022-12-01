use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::HANDLE;
use winapi::um::processthreadsapi::{OpenProcess, TerminateProcess};
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE};

pub struct WindowsProcess(HANDLE);

impl WindowsProcess 
{
    pub fn open(pid: DWORD) -> Result<WindowsProcess, String> 
    {
        let pc = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_TERMINATE, 0, pid) };
        if pc == null_mut() 
        {
            return Err("!OpenProcess".to_string());
        }
        
        Ok(WindowsProcess(pc))
    }

    pub fn kill(self) -> Result<(), String> 
    {
        unsafe { TerminateProcess(self.0, 1) };
        Ok(())
    }
}

impl Drop for WindowsProcess 
{
    fn drop(&mut self) 
    {
        unsafe { winapi::um::handleapi::CloseHandle(self.0) };
    }
}