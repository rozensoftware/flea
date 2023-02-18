#[cfg(target_os = "linux")]
use std::io::Write;
#[cfg(target_os = "linux")]
use std::{sync::mpsc, fs};
#[cfg(target_os = "linux")]
use std::thread;
#[cfg(target_os = "linux")]
use rscam::{Camera, Config};

#[cfg(target_os = "windows")]
use std::os::raw::c_char;
#[cfg(target_os = "windows")]
use std::ffi::CStr;

pub const FRAME_FILE_NAME: &str = "frame-";

#[cfg(target_os = "linux")]
pub fn save_camera_frames(number_of_frames: u32, path: &str) -> Result<(), String>
{
    //Check if camera is installed on the system
    if !fs::metadata("/dev/video0").is_ok()
    {
        return Err("Camera not found".to_string());
    }
    
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut camera = Camera::new("/dev/video0").unwrap();

        camera.start(&Config 
            {
                interval: (1, 30),
                resolution: (1280, 720),
                format: b"MJPG",
                ..Default::default()
            }).unwrap();

        for _ in 0..number_of_frames 
        {
            let frame = camera.capture().unwrap();
            tx.send(frame).unwrap();
        }
    });
    
    for i in 0..number_of_frames 
    {
        let frame = rx.recv().unwrap();
        match fs::File::create(&format!("{}/{}{}.jpg", path, FRAME_FILE_NAME, i))
        {
            Ok(mut file) => file.write_all(&frame[..]).unwrap(),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
#[link(name = "CameraLib")]
extern "C" 
{
    fn getWMV(captureTime: u32, filePath: *const c_char) -> bool;
}

#[cfg(target_os = "windows")]
pub fn save_camera_frames(_number_of_frames: u32, path: &str) -> Result<(), String>
{
    const CAPTURE_TIME: u32 = 2000;

    let mut p = format!("{}\\{}0.wmv", path, FRAME_FILE_NAME);
    unsafe {
        let arr = p.as_mut_vec();
        arr.push(0);    
        let file_path = CStr::from_bytes_with_nul_unchecked(arr).as_ptr();
        if getWMV(CAPTURE_TIME, file_path) == false
        {
            return Err("Couldn't get WMV file".to_string());
        }
    }

    Ok(())
}
