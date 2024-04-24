use crate::vec::Vec3;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color(stream: &mut impl Write, color: Color) {
    let r = (255.999 * color.x) as i32;
    let g = (255.999 * color.y) as i32;
    let b = (255.999 * color.z) as i32;
    writeln!(stream, "{} {} {}", r, g, b).unwrap();
}
