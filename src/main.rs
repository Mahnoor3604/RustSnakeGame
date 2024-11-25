extern crate piston;
extern crate piston_window;
extern crate find_folder;


use piston_window::*;
use std::time::Instant;


//need to be able to use snake struct that is created
mod game;
mod snake;
mod fruit;
mod score;
use game::Game;


pub enum GameState {
    Start,
    Playing,
    End,
}


fn main() {
    let mut highest_score: usize = 0;


    let mut window: PistonWindow = WindowSettings::new("Snake Game!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut game_state = GameState::Start;
    let mut game = Game::new();

    let mut last_frame_time = Instant::now();

    
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    println!("{:?}", assets);
    let mut glyphs = window.load_font(assets.join("hanscostudio.com.ttf")).unwrap();


    while let Some(e) = window.next() {
        let now = Instant::now();

        let delta_time = now - last_frame_time;

        last_frame_time = Instant::now();

        match game_state {
            GameState::Start=> {
                // Draw the start screen
                window.draw_2d(&e, |_c, g, _d| {
                    clear(piston_window::color::GRAY, g);

                    text::Text::new_color(piston_window::color::NAVY, 32).draw(
                    "Press any key to begin!",
                    &mut glyphs,
                    &_c.draw_state,
                    _c.transform.trans(125.0, 225.0),
                    g,
                )
                .unwrap();
                glyphs.factory.encoder.flush(_d);

                });

                if let Some(Button::Keyboard(_key)) = e.press_args() {
                    game_state = GameState::Playing; // Change to playing state
                } 
            }
            
            GameState::Playing=> {

                if let Some(button) = e.press_args() {
                    //attempting to create snake and when a button is pressed, do some action
                    game.snake.change_direction(button);
                }
        
                if !game.advance(delta_time.as_secs_f32()) {
                    game_state = GameState::End;
                }

                window.draw_2d(&e, |_c, g, _d| {
        
                    clear(piston_window::color::GRAY, g);
                    game.draw(_c, g);

                    text::Text::new_color(piston_window::color::NAVY, 20).draw(
                        game.score.print().as_str(),
                        &mut glyphs,
                        &_c.draw_state,
                        _c.transform.trans(600.0, 450.0),
                        g,
                    )
                    .unwrap();

                    text::Text::new_color(piston_window::color::NAVY, 15).draw(
                    "SCORE: ",
                    &mut glyphs,
                    &_c.draw_state,
                    _c.transform.trans(540.0, 450.0),
                    g,
                    )
                    .unwrap();
                    glyphs.factory.encoder.flush(_d); 
                
                });

            }
            
            GameState::End=> {
                if game.score.get_score() > highest_score {
                    highest_score = game.score.get_score();
                };


                window.draw_2d(&e, |_c, g, _d| {
                    clear(piston_window::color::GRAY, g);
                    text::Text::new_color(piston_window::color::NAVY, 32).draw(
                        "YOU LOST! Press space to play again.",
                        &mut glyphs,
                        &_c.draw_state,
                        _c.transform.trans(15.0, 225.0),
                        g,
                    )
                    .unwrap();

                    text::Text::new_color(piston_window::color::NAVY, 20).draw(
                        game.score.print().as_str(),
                        &mut glyphs,
                        &_c.draw_state,
                        _c.transform.trans(400.0, 275.0),
                        g,
                    )
                    .unwrap();

                    text::Text::new_color(piston_window::color::NAVY, 20).draw(
                    "YOUR SCORE WAS: ",
                    &mut glyphs,
                    &_c.draw_state,
                    _c.transform.trans(200.0, 275.0),
                    g,
                    )
                    .unwrap();
                    


                    text::Text::new_color(piston_window::color::NAVY, 20).draw(
                        "YOUR HIGHEST SCORE IS: ",
                        &mut glyphs,
                        &_c.draw_state,
                        _c.transform.trans(125.0, 325.0),
                        g,
                        )
                        .unwrap();
                        

                    text::Text::new_color(piston_window::color::NAVY, 20).draw(
                    highest_score.to_string().as_str(),
                    &mut glyphs,
                        &_c.draw_state,
                        _c.transform.trans(400.0, 325.0),
                        g,
                        )
                        .unwrap();
                    glyphs.factory.encoder.flush(_d); 
                            
                });


                if let Some(Button::Keyboard(_key)) = e.press_args() {
                    if _key == Key::Space {
                        game = Game::new();
                        game_state = GameState::Playing; // Change to playing state
                    }
                }
            }

        }
    }
}