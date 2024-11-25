use rand::Rng;
extern crate piston;
extern crate piston_window;
use piston_window::*;

//red sqaures used to represent fruit. When snake eats fruit (at same position), then changes position of fruit to ramdom position
const SQUARE_SIZE : f64 = 20.0;

pub struct Fruit {
    pub height: f32,
    pub width: f32,
}

impl Fruit {
    pub fn new(width: f32, height: f32) -> Self {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        Fruit {
            width : rng.gen_range(50.0..width-50.0),
            height : rng.gen_range(50.0..height-50.0),
        }
    }


    //make it easier to compare positions
    pub fn position(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn change_position(&mut self, width: f32, height: f32) {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        self.width = rng.gen_range(50.0..width-50.0);
        self.height = rng.gen_range(50.0..height-50.0);
    }
    
    pub fn draw(&self, _c: Context, g: &mut G2d) {

    let food_square = rectangle::rectangle_by_corners(
        self.position().0 as f64,
        self.position().1 as f64,
        self.position().0 as f64 + SQUARE_SIZE as f64,
        self.position().1 as f64 + SQUARE_SIZE as f64,
    );

    rectangle(piston_window::color::MAROON, food_square, _c.transform, g);
}

}