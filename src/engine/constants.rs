// screen constants
pub const ROW_SIZE: usize = 70; // min should be > 70
pub const COLUMNS: usize = 15; // min should be > 10

pub const WALL_VERTICAL: char = '║';
pub const WALL_HORIZONTAL: &str = "═";

pub const SNAKE_BODY: char = '■';
pub const SNAKE_HEAD: char = '⎔';
pub const SNAKE_FOOD: char = '⋇';

pub const MAX_FOOD_AVAILABLE: usize = 3;

pub const GAME_OVER_LANG: &str = "GAME OVER";
pub const RESTART_LANG: &str = "Press [ENTER] key to new game";
pub const EXIT_LANG: &str = "Press [ESC] key to exit";
pub const COMMANDS_LANG: &str = "Use your keyboard arrows to move | Press [Esc] key to exit";
