use super::{
    draw,
    snake::{Snake, SnakeDirection},
    world::World,
};
use crossterm::event::KeyCode;

#[derive(PartialEq)]
pub enum GameState {
    RUNNING,
    DEATH,
    MENU,
}

pub struct GameData {
    pub points: u16,
}

impl GameData {
    pub fn increase_points(&mut self) {
        self.points += 1;
    }
}

pub struct Game {
    data: GameData,
    world: World,
    state: GameState,
    snake: Snake,
}

impl Game {
    pub fn new_game() -> Self {
        Self {
            data: GameData { points: 0 },
            snake: Snake::create_snake(),
            state: GameState::MENU,
            world: World::create_world(),
        }
    }

    fn reset_game(&mut self) {
        self.data = GameData { points: 0 };
        self.snake = Snake::create_snake();
        self.state = GameState::RUNNING;
        self.world = World::create_world();
    }

    pub fn update(&mut self) {
        if self.state == GameState::DEATH {
            draw::draw_game_over(&self.data);
            return;
        }

        if self.state == GameState::MENU {
            draw::draw_menu();
            return;
        }

        self.snake
            .move_snake(&mut self.state, &mut self.world, &mut self.data);
        self.world.generate_food(&mut self.snake.body_position);

        draw::draw_screen(&self.snake, &mut self.world, &self.data);
    }

    pub fn move_snake_control(&mut self, key: KeyCode) {
        if self.state == GameState::DEATH || self.state == GameState::MENU {
            if key == KeyCode::Enter {
                self.reset_game();
            }

            return;
        }

        let new_direction: Option<SnakeDirection> = match key {
            KeyCode::Up if self.snake.snake_direction != SnakeDirection::DOWN => {
                Some(SnakeDirection::UP)
            }
            KeyCode::Down if self.snake.snake_direction != SnakeDirection::UP => {
                Some(SnakeDirection::DOWN)
            }
            KeyCode::Left if self.snake.snake_direction != SnakeDirection::RIGHT => {
                Some(SnakeDirection::LEFT)
            }
            KeyCode::Right if self.snake.snake_direction != SnakeDirection::LEFT => {
                Some(SnakeDirection::RIGHT)
            }
            _ => None,
        };

        if let Some(new_dir) = new_direction {
            self.snake.snake_direction = new_dir;
        }
    }
}
