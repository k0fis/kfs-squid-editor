use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone, Debug, Default)]
pub struct TextInput {
    pub content: String,
    pub cursor: usize,
}

impl TextInput {
    pub fn new(content: String) -> Self {
        let cursor = content.len();
        Self { content, cursor }
    }

    pub fn clear(&mut self) {
        self.content.clear();
        self.cursor = 0;
    }

    pub fn set(&mut self, content: String) {
        self.cursor = content.len();
        self.content = content;
    }

    pub fn value(&self) -> &str {
        &self.content
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match c {
                        'a' => self.cursor = 0,
                        'e' => self.cursor = self.content.len(),
                        'u' => {
                            self.content.drain(..self.cursor);
                            self.cursor = 0;
                        }
                        'k' => {
                            self.content.truncate(self.cursor);
                        }
                        _ => {}
                    }
                } else {
                    self.content.insert(self.cursor, c);
                    self.cursor += c.len_utf8();
                }
            }
            KeyCode::Backspace if self.cursor > 0 => {
                let prev = prev_char_boundary(&self.content, self.cursor);
                self.content.drain(prev..self.cursor);
                self.cursor = prev;
            }
            KeyCode::Delete if self.cursor < self.content.len() => {
                let next = next_char_boundary(&self.content, self.cursor);
                self.content.drain(self.cursor..next);
            }
            KeyCode::Left if self.cursor > 0 => {
                self.cursor = prev_char_boundary(&self.content, self.cursor);
            }
            KeyCode::Right if self.cursor < self.content.len() => {
                self.cursor = next_char_boundary(&self.content, self.cursor);
            }
            KeyCode::Home => self.cursor = 0,
            KeyCode::End => self.cursor = self.content.len(),
            _ => {}
        }
    }

    pub fn before_cursor(&self) -> &str {
        &self.content[..self.cursor]
    }

    pub fn after_cursor(&self) -> &str {
        &self.content[self.cursor..]
    }
}

fn prev_char_boundary(s: &str, pos: usize) -> usize {
    let mut p = pos - 1;
    while p > 0 && !s.is_char_boundary(p) {
        p -= 1;
    }
    p
}

fn next_char_boundary(s: &str, pos: usize) -> usize {
    let mut p = pos + 1;
    while p < s.len() && !s.is_char_boundary(p) {
        p += 1;
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEvent, KeyEventKind, KeyEventState};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn insert_and_move() {
        let mut input = TextInput::default();
        input.handle_key(key(KeyCode::Char('a')));
        input.handle_key(key(KeyCode::Char('b')));
        input.handle_key(key(KeyCode::Char('c')));
        assert_eq!(input.value(), "abc");
        assert_eq!(input.cursor, 3);

        input.handle_key(key(KeyCode::Left));
        input.handle_key(key(KeyCode::Left));
        assert_eq!(input.cursor, 1);

        input.handle_key(key(KeyCode::Char('X')));
        assert_eq!(input.value(), "aXbc");
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn backspace_and_delete() {
        let mut input = TextInput::new("hello".into());
        input.handle_key(key(KeyCode::Backspace));
        assert_eq!(input.value(), "hell");

        input.handle_key(key(KeyCode::Home));
        input.handle_key(key(KeyCode::Delete));
        assert_eq!(input.value(), "ell");
    }

    #[test]
    fn home_end() {
        let mut input = TextInput::new("test".into());
        input.handle_key(key(KeyCode::Home));
        assert_eq!(input.cursor, 0);
        input.handle_key(key(KeyCode::End));
        assert_eq!(input.cursor, 4);
    }
}
