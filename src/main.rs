use anyhow::Result;
use game::Game;

mod cell;
mod game;
mod input;
mod renderer;

fn main() -> Result<()> {
    let game = Game::new()?;
    renderer::render(game)
}
