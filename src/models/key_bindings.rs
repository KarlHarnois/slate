use std::fmt;

pub enum KeyBindingContext {
    Projects,
    Tasks,
    NewProject,
    NewTask,
}

pub struct KeyBinding {
    pub action: &'static str,
    pub keys: &'static [&'static str],
}

impl KeyBinding {
    fn new(action: &'static str, keys: &'static [&'static str]) -> Self {
        Self { action, keys }
    }

    pub fn list_for(context: KeyBindingContext) -> Vec<KeyBinding> {
        match context {
            KeyBindingContext::Projects => vec![
                Self::new("New", &["a"]),
                Self::new("Select", &["<space>"]),
                Self::new("Switch Table", &["<tab>"]),
                Self::new("Quit", &["<esc>", "q"]),
            ],
            KeyBindingContext::Tasks => vec![
                Self::new("New", &["a"]),
                Self::new("Toggle", &["<space>"]),
                Self::new("Switch Table", &["<tab>"]),
                Self::new("Quit", &["<esc>", "q"]),
            ],
            KeyBindingContext::NewProject => {
                vec![Self::new("Cancel", &["<esc>"])]
            }
            KeyBindingContext::NewTask => {
                vec![Self::new("Cancel", &["<esc>"])]
            }
        }
    }
}

impl fmt::Display for KeyBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keys = self.keys.join("/");
        write!(formatter, "{}: {}", self.action, keys)
    }
}
