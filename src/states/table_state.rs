use crate::states::RowState;

#[derive(Debug)]
pub enum TableType {
    Projects,
    Tasks,
}

#[derive(Debug)]
pub struct TableState {
    pub table_type: TableType,
    pub rows: Vec<RowState>,
    pub is_focused: bool,
    pub selected_row: Option<usize>,
}

impl TableState {
    pub fn new(table_type: TableType) -> Self {
        Self {
            table_type,
            rows: Vec::new(),
            is_focused: false,
            selected_row: Some(0),
        }
    }

    pub fn title(&self) -> String {
        match self.table_type {
            TableType::Projects => "Projects".to_string(),
            TableType::Tasks => "Tasks".to_string(),
        }
    }

    pub fn column_names(&self) -> Vec<String> {
        match self.table_type {
            TableType::Projects => vec![String::new(), "Name".to_string()],
            TableType::Tasks => vec!["Status".to_string(), "Name".to_string()],
        }
    }
}
