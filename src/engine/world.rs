use rand::Rng;

use super::{
    constants::{COLUMNS, MAX_FOOD_AVAILABLE, ROW_SIZE, WALL_VERTICAL},
    vec::{Screen2D, Vec2D},
};

fn is_valid_food_spawn(snake_pos: &Vec2D, rand_y: &usize, rand_x: &usize) -> bool {
    if let Some(_) = snake_pos
        .iter()
        .find(|snake_pos_ref: &&Screen2D| snake_pos_ref.y == *rand_y && snake_pos_ref.x == *rand_x)
    {
        return false;
    }

    true
}

fn create_scenario() -> Scenario {
    let mut new_scenario: Scenario = vec![vec![' '; ROW_SIZE]; COLUMNS];

    for y_pos in new_scenario.iter_mut() {
        y_pos[0] = WALL_VERTICAL;
        y_pos[ROW_SIZE - 1] = WALL_VERTICAL;
    }

    new_scenario
}

pub type Scenario = Vec<Vec<char>>;

pub struct ChangeScenario {
    pub position: Vec2D,
    pub new_value: char,
}

pub struct World {
    pub available_food: Vec2D,
    pub scenario: Scenario,
}

impl World {
    pub fn create_world() -> Self {
        Self {
            available_food: vec![],
            scenario: create_scenario(),
        }
    }

    pub fn change_scenario(&mut self, new_scene: ChangeScenario) {
        for position in &new_scene.position {
            self.scenario[position.y][position.x] = new_scene.new_value;
        }
    }

    pub fn generate_food(&mut self, snake_pos: &Vec2D) {
        let iterate_by = MAX_FOOD_AVAILABLE - self.available_food.len();

        if iterate_by == 0 {
            return;
        }

        for _ in 0..iterate_by {
            let mut rand_y = rand::thread_rng().gen_range(0..COLUMNS);
            let mut rand_x = rand::thread_rng().gen_range(1..ROW_SIZE - 1);

            while !is_valid_food_spawn(&snake_pos, &rand_y, &rand_x) {
                rand_y = rand::thread_rng().gen_range(0..COLUMNS);
                rand_x = rand::thread_rng().gen_range(1..ROW_SIZE - 1);
            }

            self.available_food.push(Screen2D::create(rand_y, rand_x));
        }
    }

    pub fn remove_food(&mut self, y: &usize, x: &usize) {
        let mut index: usize = 0;

        for (cur_index, position) in self.available_food.iter().enumerate() {
            if position.x == *x && position.y == *y {
                index = cur_index;
                break;
            }
        }

        self.available_food.remove(index);
    }
}
