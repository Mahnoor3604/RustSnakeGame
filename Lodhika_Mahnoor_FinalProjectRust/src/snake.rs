extern crate piston_window;
use piston_window::Button;
use piston::Key;
use piston_window::*;

const SQUARE_SIZE : usize = 20;


//need the derive in order to check equality
//direction used for which way the snake is going depending on user input, begins facing right
#[derive(PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

//snake is created with body that is made up of little squares
pub struct Snake {
    pub body: Vec<(f32, f32)>,
    pub direction: Direction,
    pub velocity: f32,
}

impl Snake {

    pub fn new() -> Self {
        //when initializing new snake, this will be the starting, the position and the direction will be facing right
        Snake {
            body: vec![(4.0, 2.0), (3.0, 2.0), (2.0, 2.0)],
            direction: Direction::RIGHT,
            velocity: 0.2,
        }
    }

    pub fn change_direction(&mut self, button: piston_window::Button) {
        match button {
            Button::Keyboard(Key::Up)
            if self.direction != Direction::DOWN =>
            {
                self.direction = Direction::UP
            }
            Button::Keyboard(Key::Down)
            if self.direction != Direction::UP =>
            {
                self.direction = Direction::DOWN
            }
            Button::Keyboard(Key::Right)
            if self.direction != Direction::LEFT =>
            {
                self.direction = Direction::RIGHT
            }
            Button::Keyboard(Key::Left)
            if self.direction != Direction::RIGHT =>
            {
                self.direction = Direction::LEFT
            }
            _ => return,
        }

    }

    pub fn advance(&mut self, grow: bool) -> bool {


        let head = *self.body.first().unwrap();
        let new_head = match self.direction {
            Direction::UP => (head.0, head.1 - 1.0),
            Direction::DOWN => (head.0, head.1 + 1.0),
            Direction::LEFT => (head.0  - 1.0, head.1),
            Direction::RIGHT => (head.0  + 1.0, head.1),
        };
        let new_head = (
            new_head.0,
            new_head.1
        );

        if self.body.contains(&new_head) {
            // If the head touches any body segment, return false (game over)
            return false; 
        }

        self.body.insert(0, new_head); // Insert new head at the front
        if !grow {
            self.body.pop(); // Remove the tail unless we grow
        } else {
            if self.velocity > 0.05 {
                self.velocity -= 0.01;
            } else if self.velocity > 0.01 {
                self.velocity -= 0.005;
            } else {
                self.velocity = 0.005
            }
        }
        return true;

    }

    pub fn position(&self) -> (f32, f32) {
        let head = *self.body.first().unwrap();
        return head;
    }

    pub fn draw(&self, _c: Context, g: &mut G2d) {
        
        let square_size = SQUARE_SIZE as f64;
        for square in self.body.iter() {
            let (x, y) = *square;

        // Round the x and y positions to ensure they align on the grid
        let x_pos = x as f64 * square_size;
        let y_pos = y as f64 * square_size;
            rectangle(
                piston_window::color::BLACK, 
                [
                    x_pos - 2.0, 
                    y_pos - 2.0, 
                    square_size + 4.0,
                    square_size + 4.0 
                ],
                _c.transform,
                g,
            );
            rectangle(piston_window::color::GREEN, 
                [
                    x_pos, 
                    y_pos, 
                    square_size, 
                    square_size], 
                    _c.transform, 
                    g
            );
            
        }
        
    }

}

