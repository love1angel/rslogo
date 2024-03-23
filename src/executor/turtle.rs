use super::{Color, Degree, Direction, Executor, Pixel};

#[derive(PartialEq)]
enum Status {
    Up,
    Down,
}

pub struct Turtle {
    direction: Direction,

    x: Pixel,
    y: Pixel,

    color_idx: Color,
    status: Status,

    path: std::path::PathBuf,
    image: unsvg::Image,
}

impl Turtle {
    pub fn new(width: u32, height: u32, path: std::path::PathBuf) -> Self {
        Turtle {
            direction: 0,
            x: width as f32 / 2.0,
            y: height as f32 / 2.0,
            // white
            color_idx: 7,
            status: Status::Up,
            path,
            image: unsvg::Image::new(width, height),
        }
    }

    fn draw(&mut self, pixel: Pixel) {
        if self.status == Status::Up {
            (self.x, self.y) = unsvg::get_end_coordinates(self.x, self.y, self.direction, pixel);
        } else {
            (self.x, self.y) = self
                .image
                .draw_simple_line(
                    self.x,
                    self.y,
                    self.direction,
                    pixel,
                    unsvg::COLORS[self.color_idx as usize],
                )
                // @todo error handle
                .expect("failed to write line");
        }
    }
}

impl Executor for Turtle {
    fn pen_up(&mut self) {
        println!("turtle pen_up");
        self.status = Status::Up;
    }
    fn pen_down(&mut self) {
        println!("turtle pen_down");
        self.status = Status::Down;
    }

    fn foreward(&mut self, pixel: Pixel) {
        println!("turtle foreward: {pixel}");
        if pixel < 0.0 {
            self.direction = self.direction + 180;
        }
        self.draw(pixel);
    }

    fn back(&mut self, pixel: Pixel) {
        println!("turtle back: {pixel}");
        self.foreward(-pixel);
    }

    fn left(&mut self, pixel: Pixel) {
        println!("turtle left: {pixel}");
        if pixel < 0.0 {
            self.direction = self.direction + 270;
        } else {
            self.direction = self.direction + 90;
        }
        self.draw(pixel);
    }

    fn right(&mut self, pixel: Pixel) {
        println!("turtle right: {pixel}");
        self.left(-pixel);
    }

    fn set_color(&mut self, color: Color) {
        println!("turtle set_color: {color}");
        // @todo error handle
        debug_assert!(color <= 15);
        self.color_idx = color;
    }

    fn turn(&mut self, degree: Degree) {
        println!("turtle turn: {degree}");
        if degree < 0 {
            self.direction = self.direction + degree;
        } else {
            self.direction = self.direction - degree;
        }
    }
    fn set_heading(&mut self, degree: Degree) {
        println!("turtle set_heading: {degree}");
        self.direction = degree;
    }

    fn set_x_coordinate(&mut self, coordinate: Pixel) {
        println!("turtle set_x_coordinate: {coordinate}");
        self.x = coordinate;
    }
    fn set_y_coordinate(&mut self, coordinate: Pixel) {
        println!("turtle set_y_coordinate: {coordinate}");
        self.y = coordinate;
    }

    fn get_x_coordinate(&self) -> Pixel {
        println!("turtle get_x_coordinate: {}", self.x);
        self.x
    }
    fn get_y_coordinate(&self) -> Pixel {
        println!("turtle get_y_coordinate: {}", self.y);
        self.y
    }
    fn get_heading(&self) -> Direction {
        println!("turtle get_heading: {}", self.direction);
        self.direction
    }
    fn get_color(&self) -> Color {
        println!("turtle get_color: {}", self.color_idx);
        self.color_idx
    }
}

impl Drop for Turtle {
    fn drop(&mut self) {
        match self.path.extension().map(|s| s.to_str()).flatten() {
            Some("svg") => {
                let res = self.image.save_svg(&self.path);
                if let Err(e) = res {
                    eprintln!("Error saving svg: {e}");
                    // @todo error handle
                }
            }
            Some("png") => {
                let res = self.image.save_png(&self.path);
                if let Err(e) = res {
                    eprintln!("Error saving png: {e}");
                    // @todo error handle
                }
            }
            _ => {
                eprintln!("File extension not supported");
                // @todo error handle
            }
        }
    }
}
