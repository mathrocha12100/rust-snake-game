use std::usize;

use super::{
    game::{GameData, GameState},
    logic::{sub_snake_col_pos, sub_snake_row_pos, sum_snake_col_pos, sum_snake_row_pos},
    vec::{Screen2D, Vec2D},
    world::World,
};

enum SnakeBitting {
    YOURSELF,
    FOOD,
}

#[derive(Debug, PartialEq)]
pub enum SnakeDirection {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

#[derive(Debug)]
pub struct SnakeHead {
    pub direction: SnakeDirection,
    pub position: (usize, usize), // column index, row index
}

#[derive(Debug)]
pub struct Snake {
    pub snake_direction: SnakeDirection,
    pub body_position: Vec2D,
    pub old_tail_position: Option<Screen2D>,
}

impl Snake {
    pub fn create_snake() -> Self {
        Self {
            snake_direction: SnakeDirection::RIGHT,
            body_position: vec![
                Screen2D::create(5, 3), // <- tail
                Screen2D::create(5, 4),
                Screen2D::create(5, 5),
                Screen2D::create(5, 6), // <- head
            ],
            old_tail_position: None,
        }
    }

    pub fn get_snake_tail(&self) -> Option<&Screen2D> {
        self.body_position.first()
    }

    fn walk(&mut self, new_y: usize, new_x: usize) {
        if let Some(head) = self.body_position.last_mut() {
            head.y = new_y;
            head.x = new_x;
        }
    }

    pub fn move_snake(
        &mut self,
        game_state: &mut GameState,
        world: &mut World,
        game_data: &mut GameData,
    ) {
        self.old_tail_position = Some(Screen2D {
            x: self.body_position[0].x,
            y: self.body_position[0].y,
        });

        for i in 0..self.body_position.len() - 1 {
            self.body_position[i] = self.body_position[i + 1].clone();
        }

        let mut next_y = 0;
        let mut next_x = 0;

        if let Some(head) = self.body_position.last_mut() {
            next_y = head.y;
            next_x = head.x;

            match self.snake_direction {
                SnakeDirection::UP => {
                    sub_snake_col_pos(&mut next_y, 1);
                }
                SnakeDirection::DOWN => {
                    sum_snake_col_pos(&mut next_y, 1);
                }
                SnakeDirection::LEFT => {
                    sub_snake_row_pos(&mut next_x, 1);
                }
                SnakeDirection::RIGHT => sum_snake_row_pos(&mut next_x, 1),
            }
        }

        match self.snake_is_biting(world, &next_y, &next_x) {
            Some(SnakeBitting::FOOD) => {
                world.remove_food(&next_y, &next_x);

                self.walk(next_y, next_x);
                self.feed();

                game_data.increase_points();
            }
            Some(SnakeBitting::YOURSELF) => {
                *game_state = GameState::DEATH;
            }
            None => self.walk(next_y, next_x),
        }
    }

    pub fn feed(&mut self) {
        match self.tail_is_connected_to() {
            Some(SnakeDirection::UP) => {
                let mut new_body = self.body_position[0].y.clone();

                sum_snake_col_pos(&mut new_body, 1);

                self.body_position
                    .insert(0, Screen2D::create(new_body, self.body_position[0].x))
            }
            Some(SnakeDirection::DOWN) => {
                let mut new_body = self.body_position[0].y.clone();

                sub_snake_col_pos(&mut new_body, 1);

                self.body_position
                    .insert(0, Screen2D::create(new_body, self.body_position[0].x))
            }
            Some(SnakeDirection::LEFT) => {
                let mut new_body = self.body_position[0].x.clone();

                sum_snake_row_pos(&mut new_body, 1);

                self.body_position
                    .insert(0, Screen2D::create(self.body_position[0].y, new_body))
            }
            Some(SnakeDirection::RIGHT) => {
                let mut new_body = self.body_position[0].x.clone();

                sub_snake_row_pos(&mut new_body, 1);

                self.body_position
                    .insert(0, Screen2D::create(self.body_position[0].y, new_body))
            }
            None => {}
        }
    }

    fn tail_is_connected_to(&self) -> Option<SnakeDirection> {
        if let Some(tail_position) = self.get_snake_tail() {
            let next_snake_body = &self.body_position[1];

            if tail_position.x > next_snake_body.x && tail_position.y == next_snake_body.y {
                return Some(SnakeDirection::LEFT);
            } else if tail_position.x < next_snake_body.x && tail_position.y == next_snake_body.y {
                return Some(SnakeDirection::RIGHT);
            } else if tail_position.y > next_snake_body.y {
                return Some(SnakeDirection::UP);
            } else {
                return Some(SnakeDirection::DOWN);
            }
        }

        None
    }

    fn snake_is_biting(
        &self,
        world: &World,
        next_y: &usize,
        next_x: &usize,
    ) -> Option<SnakeBitting> {
        for food_position in &world.available_food {
            if food_position.y == *next_y && food_position.x == *next_x {
                return Some(SnakeBitting::FOOD);
            }
        }

        for position in &self.body_position {
            if position.y == *next_y && position.x == *next_x {
                return Some(SnakeBitting::YOURSELF);
            }
        }

        None
    }
}
