use std::fmt;

pub enum KeyBindingContext {
    Projects,
    Tasks,
    NewProject,
    NewTask,
}

pub struct KeyBinding {
    pub action: String,
    pub keys: Vec<String>,
}

impl KeyBinding {
    pub fn list_for(context: KeyBindingContext) -> Vec<KeyBinding> {
        match context {
            KeyBindingContext::Projects => vec![
                Self::binding("New", ["a"]),
                Self::binding("Select", ["<space>"]),
                Self::binding("Switch Table", ["<tab>"]),
                Self::binding("Quit", ["<esc>", "q"]),
            ],
            KeyBindingContext::Tasks => vec![
                Self::binding("New", ["a"]),
                Self::binding("Toggle", ["<space>"]),
                Self::binding("Switch Table", ["<tab>"]),
                Self::binding("Quit", ["<esc>", "q"]),
            ],
            KeyBindingContext::NewProject => {
                vec![Self::binding("Cancel", ["<esc>"])]
            }
            KeyBindingContext::NewTask => {
                vec![Self::binding("Cancel", ["<esc>"])]
            }
        }
    }

    fn binding<'a>(action: &str, keys: impl AsRef<[&'a str]>) -> Self {
        Self {
            action: action.to_string(),
            keys: keys.as_ref().iter().map(|key| key.to_string()).collect(),
        }
    }
}

impl fmt::Display for KeyBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keys = self.keys.join("/");
        write!(formatter, "{}: {}", self.action, keys)
    }
}
