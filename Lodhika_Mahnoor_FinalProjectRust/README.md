# How to run
Simply do cargo run in order to run the main method -- no parameters or arguments needed.

# Main.rs
The main.rs file combines all the other files and updates the screen for the user to see. It creates a new instance of game and uses gameState to check if the starting screen, game screen, or end screen should be shown to the user.

# Game.rs
This file combines fruit.rs, score.rs, and snake.rs. The game keeps track of everything and calls different methods from the other files, such as snake's advance method. This ensures the game runs smoothly and everything is drawn correctly to the screen and is updated as well.

# Resources/Libraries
Piston_Window crate was used for the development for this game as well as a GFXGraphics obkect which is created through Piston_Window through the window creation. This was one of the options for game development in Rust and offered many examples. The website with the documentation itself was confusing as it didn't offer much explanation. However, there were github examples made for this exact reason which is linked through the piston_window documentation website as well as others found on github. The one I used the most can be found at this link: https://github.com/PistonDevelopers/piston-examples/blob/master/examples/hello_world.rs. Some forums on stackoverflow were also helpful such as understanding how to draw text onto the window/screen -> https://stackoverflow.com/questions/77250023/rendering-text-using-piston-in-rust.

Got font for text from: https://www.dafont.com