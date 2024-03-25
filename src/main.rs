// mod figure_circle;
// mod Parallelogram;
// use figure_circle::Circle;

mod window;
use window::{Window, Framebuffer};
use std::io;



fn main() {
    let mut window:Window = Window::new_window("Cal a object", 512, 512);

    while !window.close_window(){
        let framebuffer = window.frame_buffer();
        for x in 0..framebuffer.width(){
            for y in 0..framebuffer.height(){
                framebuffer.set_pixel(x,y,0);
            }
        }

        window.display();
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

}
