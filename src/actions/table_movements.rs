use crate::actions::Action;
use crate::states::{AppState, TableState};

pub struct MoveUpInTable;
pub struct MoveDownInTable;

impl Action for MoveUpInTable {
    fn apply(self: Box<Self>, state: &mut AppState) {
        TableMovement::perform(state.focused_table_mut(), Direction::Up);
    }
}

impl Action for MoveDownInTable {
    fn apply(self: Box<Self>, state: &mut AppState) {
        TableMovement::perform(state.focused_table_mut(), Direction::Down);
    }
}

enum Direction {
    Up,
    Down,
}

struct TableMovement<'a> {
    table: &'a mut TableState,
    direction: Direction,
}

impl<'a> TableMovement<'a> {
    pub fn perform(table: &mut TableState, direction: Direction) {
        TableMovement { table, direction }.move_row();
    }

    pub fn move_row(&mut self) {
        if self.table.rows.is_empty() {
            return;
        }

        match self.direction {
            Direction::Up => self.table.selected_row = self.previous_row(),
            Direction::Down => self.table.selected_row = self.next_row(),
        }
    }

    fn previous_row(&self) -> Option<usize> {
        Some(match self.table.selected_row {
            Some(0) | None => self.rows_len() - 1,
            Some(index) => index - 1,
        })
    }

    fn next_row(&self) -> Option<usize> {
        Some(match self.table.selected_row {
            Some(index) if index + 1 < self.rows_len() => index + 1,
            _ => 0,
        })
    }

    fn rows_len(&self) -> usize {
        self.table.rows.len()
    }
}
