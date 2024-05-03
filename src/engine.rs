use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;

use crate::engine::game::Game;

mod constants;
mod draw;
mod game;
mod logic;
mod snake;
mod vec;
mod world;

pub fn run() -> crossterm::Result<()> {
    let stdout = io::stdout();

    execute!(
        &stdout,
        EnterAlternateScreen,
        terminal::SetTitle("Rusty snake"),
    )?;

    terminal::enable_raw_mode()?;

    let mut game = Game::new_game();

    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Esc {
                    break;
                }

                game.move_snake_control(key_event.code);

                execute!(
                    &stdout,
                    terminal::Clear(terminal::ClearType::All),
                    MoveTo(0, 0)
                )?;

                game.update();
            }
        } else {
            execute!(
                &stdout,
                terminal::Clear(terminal::ClearType::All),
                MoveTo(0, 0)
            )?;

            game.update();
        }
    }

    terminal::disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
