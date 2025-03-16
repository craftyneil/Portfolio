use game::Game;

mod game;
mod player;
mod score;

fn main() {
    // std::process::Command::new("clear").status().unwrap(); // clear console

    let mut game = Game::setup();
    game.run();
}
