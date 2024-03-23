mod turtle;

// 0 <= Color <= 15
pub type Color = u32;

pub type Pixel = f32;

pub type Degree = i32;

pub type Direction = i32;

use crate::executor::turtle::Turtle;

pub trait Executor {
    fn pen_up(&mut self);
    fn pen_down(&mut self);

    fn foreward(&mut self, pixel: Pixel);
    fn back(&mut self, pixel: Pixel);
    fn left(&mut self, pixel: Pixel);
    fn right(&mut self, pixel: Pixel);

    fn set_color(&mut self, color: Color);

    fn turn(&mut self, degree: Degree);
    fn set_heading(&mut self, degree: Degree);

    fn set_x_coordinate(&mut self, pixel: Pixel);
    fn set_y_coordinate(&mut self, pixel: Pixel);

    fn get_x_coordinate(&self) -> Pixel;
    fn get_y_coordinate(&self) -> Pixel;

    fn get_heading(&self) -> Direction;
    fn get_color(&self) -> Color;
}

pub struct ExecutorFactory;

impl ExecutorFactory {
    pub fn create_turtle(width: u32, height: u32, path: std::path::PathBuf) -> Box<dyn Executor> {
        Box::new(Turtle::new(width, height, path))
    }
}
