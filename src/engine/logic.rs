use super::constants::{COLUMNS, ROW_SIZE};

pub fn sum_snake_row_pos(value: &mut usize, sum_by: usize) {
    if *value + sum_by >= ROW_SIZE - 1 {
        *value = 1;
        return;
    }

    *value += sum_by;
}

pub fn sub_snake_row_pos(value: &mut usize, sub_by: usize) {
    let res = *value as i32 - sub_by as i32;

    if res < 1 {
        *value = ROW_SIZE - 2;
        return;
    }

    *value -= sub_by;
}

pub fn sub_snake_col_pos(value: &mut usize, sub_by: usize) {
    let res = *value as i32 - sub_by as i32;

    if res < 0 {
        *value = COLUMNS - 1;
        return;
    }

    *value -= sub_by;
}

pub fn sum_snake_col_pos(value: &mut usize, sum_by: usize) {
    if *value + sum_by >= COLUMNS {
        *value = 0;
        return;
    }

    *value += sum_by;
}
