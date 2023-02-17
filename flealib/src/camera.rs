use std::io::Write;
use std::{sync::mpsc, fs};
use std::thread;

#[cfg(target_os = "linux")]
use rscam::{Camera, Config};

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
pub fn save_camera_frames(number_of_frames: u32, path: &str) -> Result<(), String>
{
    Ok(())
}