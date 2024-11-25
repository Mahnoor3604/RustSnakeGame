extern crate piston_window;

use piston_window::*;

use crate::fruit::Fruit;
use crate::snake::Snake;
use crate::score::Score;

const SQUARE_SIZE : f32 = 20.0;
const HEIGHT: f32 = 480.0;
const WIDTH: f32 = 640.0;


pub struct Game {
    pub snake: Snake,
    pub fruit: Fruit,
    pub is_eating: bool,
    pub time: f32,
    pub score: Score,
}


impl Game {
    pub fn new () -> Self {
        let snake = Snake::new();
        let fruit = Fruit::new(WIDTH, HEIGHT);
        let is_eating = false;
        let time = 0.0;
        let score = Score::new();

        Game {
            snake, fruit, is_eating, time, score
        }
    }

    pub fn advance(&mut self, delta_time: f32) -> bool {
        self.time += delta_time;

        let position: (f32, f32) = self.snake.position();
        if position.0  *SQUARE_SIZE  < 20.0 
            || position.0 *SQUARE_SIZE >= WIDTH - 20.0 
            || position.1  *SQUARE_SIZE  < 20.0
            || position.1  *SQUARE_SIZE  >= HEIGHT as f32 - 20.0 {
            return false;
        }

        self.check_food();

        if self.time > self.snake.velocity {
            
            let check: bool;

            if self.is_eating {
            
                check = self.snake.advance(true);
                self.is_eating = false;

            } else {
                check = self.snake.advance(false);
            }
            self.time = 0.0;
            return check;
    }
    return true;
        
    }

    pub fn check_food(&mut self) {

        let (head_x, head_y) = self.snake.position();
        let (fruit_x, fruit_y) = self.fruit.position();

    // Check if the snake's head is within the bounds of the fruit
        if head_x*SQUARE_SIZE <= fruit_x + SQUARE_SIZE && head_x*SQUARE_SIZE + SQUARE_SIZE  >= fruit_x && head_y*SQUARE_SIZE <= fruit_y + SQUARE_SIZE && head_y*SQUARE_SIZE + SQUARE_SIZE >= fruit_y {

            self.fruit.change_position(WIDTH, HEIGHT);

            self.is_eating = true;

            self.score.eat();
        }
    }

    pub fn draw(&mut self, _c: Context, g: &mut G2d) {

        self.snake.draw(_c, g);
        self.fruit.draw(_c, g);


        let border_color = piston_window::color::BLACK;
        let border_thickness = 20.0;
        let width: f64 = WIDTH as f64;
        let height: f64 = HEIGHT as f64;

                    
        rectangle(border_color, 
            [0.0, 0.0, width, border_thickness], 
            _c.transform, g); // Top border
        rectangle(border_color, 
            [0.0, 0.0, border_thickness, height], 
            _c.transform, g); // Left border
        rectangle(border_color, 
            [0.0, height - border_thickness, width, border_thickness], 
            _c.transform, g); // Bottom border
        rectangle(border_color, 
            [width - border_thickness, 0.0, border_thickness, height], 
            _c.transform, g); // Right border



        
                    
    }


}