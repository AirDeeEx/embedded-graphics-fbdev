#[inline(always)]
pub fn get_framebuffer_pos(x: u32, y: u32, vpp: u32, line_length: u32) -> usize {
    let pos = (y * line_length + x) * vpp;
    //(1 * 256 + 10)

    // println!(
    //     "get_framebuffer_pos: (x={}, y={}, vpp={}, line_length={})={}",
    //     x, y, vpp, line_length, pos
    // );

    pos as usize
}