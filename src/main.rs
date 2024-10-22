use crate::game::Game;

mod cell;
mod fps_counter;
mod game;
mod input;
mod window;

fn main() {
    let mut engine = console_engine::ConsoleEngine::init_fill_require(10, 10, 60).unwrap();

    let mut game = Game::new(engine.get_width(), engine.get_height());

    loop {
        engine.wait_frame();

        if game.exit {
            break;
        }

        if let Some((columns, rows)) = engine.get_resize() {
            engine.resize(columns.into(), rows.into());
            game.resize(columns, rows);
        }

        engine.clear_screen();
        input::check(&mut game, &engine);
        game.tick();
        game.draw(&mut engine);
        engine.draw();
    }
}
