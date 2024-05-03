use crate::engine::constants::{
    COMMANDS_LANG, EXIT_LANG, RESTART_LANG, ROW_SIZE, SNAKE_BODY, SNAKE_FOOD, SNAKE_HEAD,
    WALL_HORIZONTAL, WALL_VERTICAL,
};
use crate::engine::snake::Snake;

use super::constants::{COLUMNS, GAME_OVER_LANG};
use super::game::GameData;
use super::world::{ChangeScenario, World};

fn draw_snake(snake: &Snake, world: &mut World) {
    // clear old snake position
    if let Some(old_pos) = &snake.old_tail_position {
        world.change_scenario(ChangeScenario {
            new_value: ' ',
            position: vec![old_pos.clone()],
        })
    }

    let mut snake_body = snake.body_position.to_vec();
    let snake_head = snake_body.pop();

    world.change_scenario(ChangeScenario {
        new_value: SNAKE_BODY,
        position: snake_body,
    });

    if let Some(head) = snake_head {
        world.change_scenario(ChangeScenario {
            new_value: SNAKE_HEAD,
            position: vec![head],
        })
    }
}

fn draw_food(world: &mut World) {
    if world.available_food.is_empty() {
        return;
    }

    world.change_scenario(ChangeScenario {
        new_value: SNAKE_FOOD,
        position: world.available_food.to_vec(),
    })
}

fn draw_game(world: &World) {
    for column in world.scenario.iter() {
        for (i, content) in column.iter().enumerate() {
            if i == column.len() - 1 {
                println!("{}", content);
            } else {
                print!("{}", content);
            }
        }
    }
}

fn draw_x_center(text: String) {
    let mut mid_x = ((ROW_SIZE as i8 - 4 - text.len() as i8) / 2) as usize;

    if mid_x < 1 || mid_x > ROW_SIZE - 1 {
        mid_x = 1;
    }

    let final_space = ROW_SIZE - (mid_x + text.len()) - 2;

    print!("{}{}{}", " ".repeat(mid_x), text, " ".repeat(final_space));
}

pub fn draw_game_over(game_data: &GameData) {
    draw_scoreboard(game_data);

    let mid_y = (COLUMNS as i8 / 2 - 1) as usize;

    println!("╔{}╗", WALL_HORIZONTAL.repeat(ROW_SIZE - 2));

    for y in 0..COLUMNS - 1 {
        if y == mid_y {
            print!("{}", WALL_VERTICAL);
            draw_x_center(GAME_OVER_LANG.to_string());
            print!("{}\n", WALL_VERTICAL);

            print!("{}", WALL_VERTICAL);
            draw_x_center(RESTART_LANG.to_string());
            print!("{}\n", WALL_VERTICAL);

            print!("{}", WALL_VERTICAL);
            draw_x_center(EXIT_LANG.to_string());
            print!("{}\n", WALL_VERTICAL);
        } else {
            println!(
                "{}{}{}",
                WALL_VERTICAL,
                " ".repeat(ROW_SIZE - 2),
                WALL_VERTICAL
            )
        }
    }

    println!("╚{}╝", WALL_HORIZONTAL.repeat(ROW_SIZE - 2));
}

fn draw_scoreboard(game_data: &GameData) {
    println!("\n╔{}╗", WALL_HORIZONTAL.repeat(ROW_SIZE - 2));
    print!("{}", WALL_VERTICAL);

    draw_x_center(format!("Rusty snake | Points {}", game_data.points));
    print!("{}", WALL_VERTICAL);
    println!("\n╚{}╝", WALL_HORIZONTAL.repeat(ROW_SIZE - 2));
}

fn draw_commands() {
    println!("╔{}╗", WALL_HORIZONTAL.repeat(ROW_SIZE - 2));

    print!("{}", WALL_VERTICAL);
    draw_x_center(COMMANDS_LANG.to_string());
    print!("{}", WALL_VERTICAL);

    println!("\n╚{}╝", WALL_HORIZONTAL.repeat(ROW_SIZE - 2));
}

pub fn draw_menu() {
    let mid_y = (COLUMNS as i8 / 2 - 1) as usize;

    println!("\n╔{}╗", WALL_HORIZONTAL.repeat(ROW_SIZE - 2));
    print!("{}", WALL_VERTICAL);

    draw_x_center(String::from("Rusty snake"));
    print!("{}", WALL_VERTICAL);
    println!("\n╚{}╝", WALL_HORIZONTAL.repeat(ROW_SIZE - 2));

    println!("╔{}╗", WALL_HORIZONTAL.repeat(ROW_SIZE - 2));

    for y in 0..COLUMNS - 1 {
        if y == mid_y {
            print!("{}", WALL_VERTICAL);
            draw_x_center(RESTART_LANG.to_string());
            print!("{}\n", WALL_VERTICAL);
        } else {
            println!(
                "{}{}{}",
                WALL_VERTICAL,
                " ".repeat(ROW_SIZE - 2),
                WALL_VERTICAL
            )
        }
    }

    println!("╚{}╝", WALL_HORIZONTAL.repeat(ROW_SIZE - 2));
    draw_commands();
}

pub fn draw_screen(snake: &Snake, world: &mut World, game_data: &GameData) {
    draw_snake(snake, world);
    draw_food(world);

    draw_scoreboard(game_data);

    println!("╔{}╗", WALL_HORIZONTAL.repeat(ROW_SIZE - 2));

    draw_game(&world);

    println!("╚{}╝", WALL_HORIZONTAL.repeat(ROW_SIZE - 2));
    draw_commands();
}
