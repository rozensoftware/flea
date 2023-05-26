use log::{error, debug};
use scrap::{Capturer, Display};
use std::time::Duration;
use std::{str, fs::File, thread, io::ErrorKind::WouldBlock};

pub struct Screenshot
{
}

impl Default for Screenshot
{
    fn default() -> Self
    {
        Self::new()
    }
}

impl Screenshot
{
    pub fn new() -> Screenshot
    {
        Screenshot
        {

        }
    }

    /// Takes screenshot and save it as a PNG file in a passed file
    /// * file_path - a path with a filename to store the screenshot
    pub fn take_screenshot(&self, file_path: &str) -> Result<(), String>
    {
        let one_second = Duration::new(1, 0);
        let one_frame = one_second / 60;
    
        let display = match Display::primary()
        {
            Ok(x) =>
            {
                x
            },
            Err(y) =>
            {
                error!("{}", y.to_string());
                return Err(y.to_string())
            }
        };

        let mut capturer = match Capturer::new(display)
        {
            Ok(x) =>
            {
                x
            },
            Err(y) =>
            {
                error!("{}", y.to_string());
                return Err(y.to_string())
            }
        };

        let (w, h) = (capturer.width(), capturer.height());
    
        loop 
        {
            // Wait until there's a frame.
    
            let buffer = match capturer.frame() 
            {
                Ok(buffer) => buffer,
                Err(error) => 
                {
                    if error.kind() == WouldBlock 
                    {
                        // Keep spinning.
                        thread::sleep(one_frame);
                        continue;
                    } 
                    else 
                    {
                        let e = std::io::Error::new(std::io::ErrorKind::Other, "Exception while sleeping in thread");
                        error!("{}", e.to_string());
                        return Err(e.to_string());
                    }
                }
            };
    
            debug!("Screen captured! Saving...");
    
            // Flip the ARGB image into a BGRA image.
    
            let mut bitflipped = Vec::with_capacity(w * h * 4);
            let stride = buffer.len() / h;
    
            for y in 0..h 
            {
                for x in 0..w 
                {
                    let i = stride * y + 4 * x;
                    bitflipped.extend_from_slice(&[
                        buffer[i + 2],
                        buffer[i + 1],
                        buffer[i],
                        255,
                    ]);
                }
            }
    
            // Save the image.
    
            match repng::encode(
                File::create(file_path).unwrap(),
                w as u32,
                h as u32,
                &bitflipped,)
                {
                    Ok(_) =>
                    {
                    },
                    Err(x) =>
                    {
                        error!("{}", x.to_string());
                        return Err(x.to_string());
                    }
                }
    
            debug!("Image saved.");
            break;
        }

        Ok(())
    }
}