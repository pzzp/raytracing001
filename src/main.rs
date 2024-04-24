use std::io;

use color::Color;
use ray::Ray;
use vec::Point3;

use crate::{color::write_color, vec::Vec3};

mod color;
mod ray;
mod vec;

fn ray_color(ray: Ray) -> Color {
    let unit_direction = ray.dir.to_unit_vec3();
    let a = (unit_direction.y + 1.0) * 0.5;
    (1.0 - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    let asepct_ratio = 16.0 / 9.0;
    let image_width = 256;
    let image_height = {
        let iw = image_width as f64;
        let ih = iw / asepct_ratio;
        if ih < 1.0 {
            1
        } else {
            ih as i32
        }
    };

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let camera_center = Point3::new(0., 0., 0.);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel100_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render

    println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel100_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(r);
            write_color(&mut io::stdout(), pixel_color);
        }
    }
}
