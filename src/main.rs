use anyhow::Result;
use game::Game;

mod cell;
mod game;
mod input;
mod renderer;

fn main() -> Result<()> {
    let game = Game::new(60, 3)?;

    renderer::render(game)
}
