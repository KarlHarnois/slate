#[derive(Debug)]
pub struct TextInput {
    pub value: String,
    pub cursor: usize,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
        }
    }

    pub fn insert(&mut self, char: char) {
        self.value.insert(self.cursor, char);
        self.cursor += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor == 0 {
            return;
        }

        let before_char_to_delete = self.value.chars().take(self.cursor - 1);
        let after_char_to_delete = self.value.chars().skip(self.cursor);

        self.value =
            before_char_to_delete.chain(after_char_to_delete).collect();

        self.cursor -= 1;
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor == 0 {
            return;
        };
        self.cursor -= 1;
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor == self.value.len() {
            return;
        }
        self.cursor += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::states::TextInput;

    #[test]
    fn insert() {
        let mut input = TextInput {
            value: "test".to_string(),
            cursor: 2,
        };

        input.insert('a');

        assert_eq!(
            input.value,
            "teast".to_string(),
            "Char is inserted at the cursor"
        );

        assert_eq!(input.cursor, 3, "Cursor is moved");
    }

    #[test]
    fn delete_char() {
        let mut input = TextInput {
            value: "test".to_string(),
            cursor: 2,
        };

        input.delete_char();

        assert_eq!(
            input.value,
            "tst".to_string(),
            "Char is deleted at the cursor"
        );

        assert_eq!(input.cursor, 1, "Cursor is moved");
    }

    #[test]
    fn move_cursor_left() {
        let mut input = TextInput {
            value: "test".to_string(),
            cursor: 1,
        };

        input.move_cursor_left();
        assert_eq!(input.cursor, 0, "Cursor is decremented");

        input.move_cursor_left();
        assert_eq!(input.cursor, 0, "Cursor cannot move pass minimum");
    }

    #[test]
    fn move_cursor_right() {
        let mut input = TextInput {
            value: "test".to_string(),
            cursor: 3,
        };

        input.move_cursor_right();
        assert_eq!(input.cursor, 4, "Cursor is incremented");

        input.move_cursor_right();
        assert_eq!(input.cursor, 4, "Cursor cannot move pass maximum");
    }
}
