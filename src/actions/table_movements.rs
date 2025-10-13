use crate::actions::Action;
use crate::states::{AppState, TableState};

pub struct MoveUpInTable;
pub struct MoveDownInTable;

impl Action for MoveUpInTable {
    fn apply(self: Box<Self>, state: &mut AppState) {
        self.select_previous_row_if_focused(&mut state.projects_table);
        self.select_previous_row_if_focused(&mut state.tasks_table);
    }
}

impl MoveUpInTable {
    fn select_previous_row_if_focused(&self, table: &mut TableState) {
        if table.is_focused {
            table.selected_row = self.previous_row(table);
        }
    }

    fn previous_row(&self, table: &TableState) -> Option<usize> {
        if table.rows.is_empty() {
            None
        } else {
            Some(match table.selected_row {
                Some(0) | None => table.rows.len() - 1,
                Some(index) => index - 1,
            })
        }
    }
}

impl Action for MoveDownInTable {
    fn apply(self: Box<Self>, state: &mut AppState) {
        self.select_next_row_if_focused(&mut state.projects_table);
        self.select_next_row_if_focused(&mut state.tasks_table);
    }
}

impl MoveDownInTable {
    fn select_next_row_if_focused(&self, table: &mut TableState) {
        if table.is_focused {
            table.selected_row = self.next_row(table);
        }
    }

    fn next_row(&self, table: &TableState) -> Option<usize> {
        if table.rows.is_empty() {
            None
        } else {
            Some(match table.selected_row {
                Some(index) if index + 1 < table.rows.len() => index + 1,
                _ => 0,
            })
        }
    }
}
