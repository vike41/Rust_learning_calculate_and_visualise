use minifb::WindowOptions;

pub struct Window{
    window: minifb::Window,
    framebuffer: Framebuffer
}

pub struct Framebuffer{
    data: Vec<u32>,
    width: usize,
    height: usize
}

impl Window {
    pub fn new_window(name: &str, screen_width: usize, screen_height: usize) -> Self{
        let options: WindowOptions = minifb::WindowOptions{
            resize: true,
            ..Default::default()
        };

        let window = minifb::Window::new(
            name,
            screen_width,
            screen_height,
            options
        ).expect("Failed to create window!");

        Window{
            window,
            framebuffer:Framebuffer::new(screen_width, screen_height)
        }
    }
    pub fn frame_buffer(&mut self) -> &mut Framebuffer{
        &mut self.framebuffer
    }
    pub fn close_window (&self) -> bool {
        !self.window.is_open()
    }

    pub fn display(&mut self){
        self.window.update_with_buffer(
            &self.framebuffer.data,
            self.framebuffer.width,
            self.framebuffer.height
        ).expect("Failed to display window pixels!");
    }
}
impl Framebuffer{
    pub fn new_frame_buffer(width : usize, height : usize) -> Self{
        Framebuffer{
            data:
            vec![0; width * height],
            width,
            height
        }
    }

    pub fn width(&self) -> usize{
        self.width
    }

    pub fn height(&self) -> usize{
        self.height
    }

    pub fn set_pixel(&mut self, x:usize, y:usize, value: u32){
        self.data[x + y * self.width] = value;
    }


}
