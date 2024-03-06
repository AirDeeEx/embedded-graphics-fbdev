use std::convert::Infallible;
use std::path::Path;

use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    Pixel,
    pixelcolor::{Rgb888, RgbColor}
};
use crate::fbdev_raw::Fbdev;


pub struct FbdevDisplay {
    pub raw: Fbdev,
    size: Size,
    pub pixel_count: u32,
}

impl FbdevDisplay {
    pub fn new(path: &Path) -> FbdevDisplay {
        let fbdev = Fbdev::init(path);
        let (fb_w,fb_h) = fbdev.get_size();
        FbdevDisplay {
            raw: fbdev,
            size: Size::new(fb_w,fb_h),
            pixel_count: 0,
        }
    }
    
    pub fn flush(&mut self) {
        self.raw.flush();
        self.pixel_count = 0;
    }

    pub fn reset(&mut self) {
        self.raw.clear();
    }

    pub fn pan_display_current(&mut self) {
        self.raw.pan_display_current();
    }
}

impl OriginDimensions for FbdevDisplay {
    fn size(&self) -> Size {
        self.size
    }
}

impl DrawTarget for FbdevDisplay {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(pos, color) in pixels.into_iter() {
            if let Ok((x, y)) = <(u32, u32)>::try_from(pos) {
                if !(x > self.size.width || y > self.size.height)
                {
                    self.raw.set_pixel(x,y,color.r(),color.g(),color.b());
                    self.pixel_count += 1;
                }
            }
        }

        Ok(())
    }
}