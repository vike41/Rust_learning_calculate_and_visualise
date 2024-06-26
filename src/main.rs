// mod figure_circle;
// mod Parallelogram;
// use figure_circle::Circle;

mod window;
use window::{Window, Framebuffer};
use std::io;

use glam::*;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn edge_function(a: &Vec2, c: &Vec2, b: &Vec2) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}

fn inside_triangle(a: &Vec2, b: &Vec2, c: &Vec2, p: &Vec2) -> bool {
    let a0 = edge_function(b, c, p);
    let a1 = edge_function(c, a, p);
    let a2 = edge_function(a, b, p);

    let mut overlaps = true;
    overlaps &= a0 > 0.0;
    overlaps &= a1 > 0.0;
    overlaps &= a2 > 0.0;

    overlaps
}

fn draw_triangle(framebuffer: &mut Framebuffer, a: &Vec2, b: &Vec2, c: &Vec2, color: u32) {
    let width = framebuffer.width();
    let height = framebuffer.height();

    let min = a.min(b.min(*c)).max(Vec2::ZERO) * Vec2::new(width as f32, height as f32);
    let max = a.max(b.max(*c)).min(Vec2::ONE) * Vec2::new(width as f32, height as f32);

    for x in (min.x as usize)..(max.x as usize) {
        for y in (min.y as usize)..(max.y as usize) {
            let p = Vec2::new(
                x as f32 / width as f32,
                y as f32 / height as f32
            );

            let inside = inside_triangle(a, b, c, &p);
            if inside {
                framebuffer.set_pixel(x, y, color);
            }
        }
    }
}
static TRIANGLE_GREEN: &[Vec2] = &[
    Vec2::new(0.4, 0.3),
    Vec2::new(0.7, 0.3),
    Vec2::new(0.2, 0.7),
];

static TRIANGLE_RED: &[Vec2] = &[
    Vec2::new(0.1, 0.3),
    Vec2::new(0.5, 0.1),
    Vec2::new(0.2, 0.6),
];

static TRIANGLE_BLUE: &[Vec2] = &[
    Vec2::new(0.5, 0.7),
    Vec2::new(0.9, 0.7),
    Vec2::new(0.5, 0.9),
];
static QUAD_WHITE: &[Vec2] = &[
    Vec2::new(0.2, 0.2),
    Vec2::new(0.4, 0.5),
    Vec2::new(0.6, 0.6),
    Vec2::new(0.7, 0.7),
];
static POINTS: &[Vec2] = &[
    TRIANGLE_GREEN[0],
    TRIANGLE_GREEN[1],
    TRIANGLE_GREEN[2],

    TRIANGLE_RED[0],
    TRIANGLE_RED[1],
    TRIANGLE_RED[2],

    TRIANGLE_BLUE[0],
    TRIANGLE_BLUE[1],
    TRIANGLE_BLUE[2],

    QUAD_WHITE[0],
    QUAD_WHITE[1],
    QUAD_WHITE[2],
    QUAD_WHITE[3],

];

fn main() {
    let mut window = Window::new("3D graphics from scratch! (PART 1)", 256, 256);

    while !window.should_close() {
        let framebuffer = window.framebuffer();

        framebuffer.clear(from_u8_rgb(20, 20, 20));

        for i in 0..(POINTS.len() / 3) {
            draw_triangle(
                framebuffer,
                &POINTS[i * 3],
                &POINTS[i * 3 + 1],
                &POINTS[i * 3 + 2],
                from_u8_rgb((i * 100 + 100) as u8, 100, 50)
            );
        }

        window.display();
    }
}

    // // Define Circle
    // let mut input = String::new();
    //
    // println!("Enter Radius for Circle  ---");
    // io::stdin().read_line(&mut input).expect("Failed to read line");
    // let in_radius: f64 = input.trim().parse().expect("Invalid number");
    // input.clear();
    //
    // println!("Enter angle for Circle in degrees: ");
    // io::stdin().read_line(&mut input).expect("Failed to red line");
    // let in_angle = input.trim().parse().expect("Not valid number");
    //
    // let mut circ = Circle{
    //     radius: in_radius,
    //     angle: in_angle,
    // };
    //
    // let diameter = circ.cal_diameter();
    // let circle_length = circ.cal_circle_length();
    // let area_sector = circ.cal_sectors_area();
    // let area_segment = circ.cal_segments_area();
    //
    // println!("Input Number for radius is: {} and for angle {}", circ.radius, circ.angle);
    // println!("Circles Diameter is {}, length is: {:.0}", diameter, circle_length);
    // println!("Area sector is: {:.0} and area Segment is: {:.0}", area_sector, area_segment);


