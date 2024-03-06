use std::path::Path;
use std::result;

use framebuffer::{Framebuffer, FramebufferError};
use rgb565::Rgb565;

use crate::common::get_framebuffer_pos;

pub enum FbType {
    RGB565,
}

pub struct Fbdev {
    pub(crate) framebuffer: Framebuffer,
    fb_type: FbType,
    frame: Vec<u8>,
}

impl Fbdev {
    pub fn init(path: &Path) -> Fbdev {
        let mut fb = Framebuffer::new(path).unwrap();

        println!(
            "Framebuffer : res={}x{} (virtual {}x{}) ({}x{} mm), {} bytes",
            fb.var_screen_info.xres,
            fb.var_screen_info.yres,
            fb.var_screen_info.xres_virtual,
            fb.var_screen_info.yres_virtual,
            fb.var_screen_info.width,
            fb.var_screen_info.height,
            fb.frame.len()
        );
        println!("DEBUG fb_var: {:?}", fb.var_screen_info);
        println!("DEBUG fb_fix: {:?}", fb.fix_screen_info);

        println!("set yoffset=0");
        fb.var_screen_info.yoffset = 0;
        Framebuffer::put_var_screeninfo(&fb.device, &fb.var_screen_info).unwrap();
        println!("set yoffset=0 Ok!");

        //&mut fb.frame[..fb.frame.len()/2];

        let frame = vec![0; fb.frame.len()];

        //fb.write_frame(&frame);

        Fbdev {
            framebuffer: fb,
            fb_type: FbType::RGB565,
            frame: frame,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) -> Option<()> {
        match self.fb_type {
            FbType::RGB565 => {
                self.set_pixel_rgb565(x, y, r, g, b);
            }
        }
        Some(())
    }

    pub fn clear(&mut self) {
        let last_index = get_framebuffer_pos(
            self.framebuffer.var_screen_info.xres,
            self.framebuffer.var_screen_info.yres,
            2,
            self.framebuffer.var_screen_info.xres,
        );
        self.frame[0..last_index].fill(0);
    }

    pub fn flush(&mut self) {
        // let last_index = get_framebuffer_pos(
        //     self.framebuffer.var_screen_info.xres,
        //     self.framebuffer.var_screen_info.yres,
        //     2,
        //     self.framebuffer.var_screen_info.xres,
        // );

        self.framebuffer.write_frame(&self.frame);
        //self.framebuffer.frame[..last_index].copy_from_slice(&self.frame[..last_index]); //TODO: copy only active page
        // Framebuffer::pan_display(&self.framebuffer.device, &self.framebuffer.var_screen_info)
        //     .unwrap();
    }

    pub fn pan_display_current(&mut self) -> result::Result<i32, FramebufferError> {
        Framebuffer::pan_display(&self.framebuffer.device, &self.framebuffer.var_screen_info)
    }

    pub fn get_size(&self) -> (u32, u32) {
        (
            self.framebuffer.var_screen_info.xres,
            self.framebuffer.var_screen_info.yres,
        )
    }

    fn set_pixel_rgb565(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        let rgb565 = Rgb565::from_rgb888_components(r, g, b);

        let parts = rgb565.to_rgb565_le();

        let pos_start = crate::common::get_framebuffer_pos(x, y, 2, 256);

        self.frame[pos_start + 0] = parts[0];
        self.frame[pos_start + 1] = parts[1];
    }
}