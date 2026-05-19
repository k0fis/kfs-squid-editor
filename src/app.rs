use std::path::PathBuf;
use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::TableState;

use crate::input::TextInput;
use crate::model::*;

const MENU_STRUCTURE: &[(&str, &[(&str, &str)])] = &[
    ("File", &[("Save", "Ctrl+S"), ("Quit", "Ctrl+Q")]),
    (
        "Edit",
        &[
            ("Undo", "Ctrl+Z"),
            ("Redo", "Ctrl+Y"),
            ("Add", "a"),
            ("Edit", "e"),
            ("Delete", "d"),
        ],
    ),
    (
        "View",
        &[
            ("Rules", ""),
            ("Auth", ""),
            ("Direct", ""),
            ("Search", "/"),
        ],
    ),
    ("Help", &[("Help", "F1")]),
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tab {
    Rules,
    Auth,
    Direct,
}

impl Tab {
    pub fn next(&self) -> Self {
        match self {
            Self::Rules => Self::Auth,
            Self::Auth => Self::Direct,
            Self::Direct => Self::Rules,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Rules => Self::Direct,
            Self::Auth => Self::Rules,
            Self::Direct => Self::Auth,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Rules => "ACLs & Rules",
            Self::Auth => "Auth",
            Self::Direct => "Direct",
        }
    }
}

pub const ALL_TABS: &[Tab] = &[Tab::Rules, Tab::Auth, Tab::Direct];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Screen {
    List,
    AclEdit { index: Option<usize> },
    AccessEdit { index: Option<usize> },
    DirectEdit { always: bool, index: Option<usize> },
    ConfirmQuit,
    ConfirmDelete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputField {
    Name,
    Type,
    Values,
    CaseInsensitive,
    Action,
    AclPicker,
    AuthProgram,
    AuthChildren,
    AuthRealm,
    AuthTtl,
}

pub struct App {
    pub config: SquidConfig,
    pub screen: Screen,
    pub tab: Tab,
    pub should_quit: bool,
    pub file_path: PathBuf,
    pub dirty: bool,
    pub status_message: Option<(String, Instant)>,
    pub help_visible: bool,

    // Menu bar (F9)
    pub menu_active: bool,
    pub menu_index: usize,
    pub menu_item: usize,

    // Undo/redo
    undo_stack: Vec<SquidConfig>,
    redo_stack: Vec<SquidConfig>,

    // Rules tab: top (ACLs) / bottom (http_access) focus
    pub rules_focus_acls: bool,
    pub acl_table_state: TableState,
    pub access_table_state: TableState,
    pub acl_filter: Option<TextInput>,

    // Direct tab
    pub always_direct_state: TableState,
    pub never_direct_state: TableState,
    pub direct_focus_always: bool,

    // ACL edit
    pub edit_name: TextInput,
    pub edit_type_index: usize,
    pub edit_values: TextInput,
    pub edit_case_insensitive: bool,
    pub edit_field: InputField,

    // Access/Direct edit
    pub access_action: AccessAction,
    pub access_acl_refs: Vec<AclRef>,
    pub access_available_cursor: usize,
    pub access_selected_cursor: usize,
    pub access_focus_available: bool,

    // Auth edit
    pub auth_program: TextInput,
    pub auth_children: TextInput,
    pub auth_realm: TextInput,
    pub auth_ttl: TextInput,
    pub auth_field: InputField,
}

impl App {
    pub fn new(config: SquidConfig, file_path: PathBuf) -> Self {
        let mut app = Self {
            config,
            screen: Screen::List,
            tab: Tab::Rules,
            should_quit: false,
            file_path,
            dirty: false,
            status_message: None,
            help_visible: false,
            menu_active: false,
            menu_index: 0,
            menu_item: 0,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            rules_focus_acls: true,
            acl_table_state: TableState::default(),
            access_table_state: TableState::default(),
            acl_filter: None,
            always_direct_state: TableState::default(),
            never_direct_state: TableState::default(),
            direct_focus_always: true,
            edit_name: TextInput::default(),
            edit_type_index: 0,
            edit_values: TextInput::default(),
            edit_case_insensitive: false,
            edit_field: InputField::Name,
            access_action: AccessAction::Allow,
            access_acl_refs: Vec::new(),
            access_available_cursor: 0,
            access_selected_cursor: 0,
            access_focus_available: true,
            auth_program: TextInput::default(),
            auth_children: TextInput::default(),
            auth_realm: TextInput::default(),
            auth_ttl: TextInput::default(),
            auth_field: InputField::AuthProgram,
        };
        app.load_auth_fields();
        if !app.config.acls.is_empty() {
            app.acl_table_state.select(Some(0));
        }
        if !app.config.http_access.is_empty() {
            app.access_table_state.select(Some(0));
        }
        if !app.config.always_direct.is_empty() {
            app.always_direct_state.select(Some(0));
        }
        if !app.config.never_direct.is_empty() {
            app.never_direct_state.select(Some(0));
        }
        app
    }

    fn load_auth_fields(&mut self) {
        self.auth_program =
            TextInput::new(self.config.auth_param.program.clone().unwrap_or_default());
        self.auth_children =
            TextInput::new(self.config.auth_param.children.clone().unwrap_or_default());
        self.auth_realm = TextInput::new(self.config.auth_param.realm.clone().unwrap_or_default());
        self.auth_ttl = TextInput::new(
            self.config
                .auth_param
                .credentialsttl
                .clone()
                .unwrap_or_default(),
        );
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some((msg.into(), Instant::now()));
    }

    fn snapshot(&mut self) {
        self.undo_stack.push(self.config.clone());
        self.redo_stack.clear();
        if self.undo_stack.len() > 50 {
            self.undo_stack.remove(0);
        }
    }

    fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.config.clone());
            self.config = prev;
            self.dirty = true;
            self.load_auth_fields();
            self.set_status("Undo");
        }
    }

    fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.config.clone());
            self.config = next;
            self.dirty = true;
            self.load_auth_fields();
            self.set_status("Redo");
        }
    }

    pub fn menu_items(&self) -> &[(&'static str, &'static [(&'static str, &'static str)])] {
        MENU_STRUCTURE
    }

    fn handle_menu_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::F(9) => self.menu_active = false,
            KeyCode::Left => {
                if self.menu_index == 0 {
                    self.menu_index = MENU_STRUCTURE.len() - 1;
                } else {
                    self.menu_index -= 1;
                }
                self.menu_item = 0;
            }
            KeyCode::Right => {
                self.menu_index = (self.menu_index + 1) % MENU_STRUCTURE.len();
                self.menu_item = 0;
            }
            KeyCode::Up => {
                let item_count = MENU_STRUCTURE[self.menu_index].1.len();
                if self.menu_item == 0 {
                    self.menu_item = item_count - 1;
                } else {
                    self.menu_item -= 1;
                }
            }
            KeyCode::Down => {
                let item_count = MENU_STRUCTURE[self.menu_index].1.len();
                self.menu_item = (self.menu_item + 1) % item_count;
            }
            KeyCode::Enter => {
                self.execute_menu_action();
                self.menu_active = false;
            }
            _ => {}
        }
    }

    fn execute_menu_action(&mut self) {
        match (self.menu_index, self.menu_item) {
            // File
            (0, 0) => self.save_config(),
            (0, 1) => {
                if self.dirty {
                    self.screen = Screen::ConfirmQuit;
                } else {
                    self.should_quit = true;
                }
            }
            // Edit
            (1, 0) => self.undo(),
            (1, 1) => self.redo(),
            (1, 2) => self.start_add(),
            (1, 3) => self.start_edit(),
            (1, 4) if self.has_selection() => {
                self.screen = Screen::ConfirmDelete;
            }
            // View
            (2, 0) => self.tab = Tab::Rules,
            (2, 1) => self.tab = Tab::Auth,
            (2, 2) => self.tab = Tab::Direct,
            (2, 3) if self.tab == Tab::Rules && self.rules_focus_acls => {
                self.acl_filter = Some(TextInput::default());
            }
            // Help
            (3, 0) => self.help_visible = true,
            _ => {}
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if let Some((_, time)) = &self.status_message
            && time.elapsed().as_secs() > 3
        {
            self.status_message = None;
        }

        if key.code == KeyCode::F(9) {
            self.menu_active = !self.menu_active;
            self.menu_index = 0;
            self.menu_item = 0;
            return;
        }

        if self.menu_active {
            self.handle_menu_key(key);
            return;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('q') => {
                    if self.dirty {
                        self.screen = Screen::ConfirmQuit;
                    } else {
                        self.should_quit = true;
                    }
                    return;
                }
                KeyCode::Char('s') => {
                    self.save_config();
                    return;
                }
                KeyCode::Char('z') => {
                    self.undo();
                    return;
                }
                KeyCode::Char('y') => {
                    self.redo();
                    return;
                }
                _ => {}
            }
        }

        if self.help_visible {
            match key.code {
                KeyCode::Esc | KeyCode::Char('?') | KeyCode::F(1) => {
                    self.help_visible = false;
                }
                _ => {}
            }
            return;
        }

        match &self.screen.clone() {
            Screen::ConfirmQuit => match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => self.should_quit = true,
                _ => self.screen = Screen::List,
            },
            Screen::ConfirmDelete => match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.do_delete();
                    self.screen = Screen::List;
                }
                _ => self.screen = Screen::List,
            },
            Screen::List => self.handle_list_key(key),
            Screen::AclEdit { .. } => self.handle_acl_edit_key(key),
            Screen::AccessEdit { .. } => self.handle_access_edit_key(key),
            Screen::DirectEdit { .. } => self.handle_access_edit_key(key),
        }
    }

    fn handle_list_key(&mut self, key: KeyEvent) {
        if self.acl_filter.is_some() {
            match key.code {
                KeyCode::Esc => {
                    self.acl_filter = None;
                }
                KeyCode::Enter => {
                    // Keep filter active but stop editing
                    // (pressing / again will resume editing)
                }
                _ => {
                    self.acl_filter.as_mut().unwrap().handle_key(key);
                    self.acl_table_state
                        .select(if self.filtered_acl_indices().is_empty() {
                            None
                        } else {
                            Some(0)
                        });
                }
            }
            return;
        }

        match key.code {
            KeyCode::Char('?') | KeyCode::F(1) => self.help_visible = true,
            KeyCode::Tab => match self.tab {
                Tab::Rules => {
                    self.rules_focus_acls = !self.rules_focus_acls;
                }
                Tab::Direct => {
                    self.direct_focus_always = !self.direct_focus_always;
                }
                _ => {}
            },
            KeyCode::BackTab => self.tab = self.tab.prev(),
            KeyCode::Esc => self.tab = self.tab.next(),
            KeyCode::Char('q') => {
                if self.dirty {
                    self.screen = Screen::ConfirmQuit;
                } else {
                    self.should_quit = true;
                }
            }
            KeyCode::Char('/') if self.tab == Tab::Rules && self.rules_focus_acls => {
                self.acl_filter = Some(TextInput::default());
            }
            KeyCode::Char('j') | KeyCode::Down => self.move_selection(1),
            KeyCode::Char('k') | KeyCode::Up => self.move_selection(-1),
            KeyCode::Char('a') => self.start_add(),
            KeyCode::Char('e') | KeyCode::Enter => self.start_edit(),
            KeyCode::Char('d') if self.has_selection() => {
                self.screen = Screen::ConfirmDelete;
            }
            KeyCode::Char('u') => self.move_rule(-1),
            KeyCode::Char('J') => self.move_rule(1),
            _ => {}
        }
    }

    fn handle_acl_edit_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.screen = Screen::List,
            KeyCode::Tab => {
                self.edit_field = match self.edit_field {
                    InputField::Name => InputField::Type,
                    InputField::Type => InputField::Values,
                    InputField::Values => InputField::CaseInsensitive,
                    InputField::CaseInsensitive => InputField::Name,
                    _ => InputField::Name,
                };
            }
            KeyCode::BackTab => {
                self.edit_field = match self.edit_field {
                    InputField::Name => InputField::CaseInsensitive,
                    InputField::Type => InputField::Name,
                    InputField::Values => InputField::Type,
                    InputField::CaseInsensitive => InputField::Values,
                    _ => InputField::Name,
                };
            }
            KeyCode::F(2) => {
                self.save_acl_edit();
            }
            _ => match self.edit_field {
                InputField::Name => self.edit_name.handle_key(key),
                InputField::Type => match key.code {
                    KeyCode::Left => {
                        if self.edit_type_index > 0 {
                            self.edit_type_index -= 1;
                        } else {
                            self.edit_type_index = AclType::ALL.len() - 1;
                        }
                    }
                    KeyCode::Right => {
                        self.edit_type_index = (self.edit_type_index + 1) % AclType::ALL.len();
                    }
                    _ => {}
                },
                InputField::Values => self.edit_values.handle_key(key),
                InputField::CaseInsensitive
                    if (key.code == KeyCode::Char(' ') || key.code == KeyCode::Enter) =>
                {
                    self.edit_case_insensitive = !self.edit_case_insensitive;
                }
                _ => {}
            },
        }
    }

    fn handle_access_edit_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.screen = Screen::List,
            KeyCode::Tab => {
                self.edit_field = match self.edit_field {
                    InputField::Action => InputField::AclPicker,
                    InputField::AclPicker => InputField::Action,
                    _ => InputField::Action,
                };
            }
            KeyCode::F(2) => {
                self.save_access_edit();
            }
            _ => match self.edit_field {
                InputField::Action
                    if (key.code == KeyCode::Left
                        || key.code == KeyCode::Right
                        || key.code == KeyCode::Char(' ')) =>
                {
                    self.access_action = match self.access_action {
                        AccessAction::Allow => AccessAction::Deny,
                        AccessAction::Deny => AccessAction::Allow,
                    };
                }
                InputField::AclPicker => match key.code {
                    KeyCode::Char('j') | KeyCode::Down => {
                        if self.access_focus_available {
                            let max = self.available_acl_names().len();
                            if max > 0 {
                                self.access_available_cursor =
                                    (self.access_available_cursor + 1).min(max - 1);
                            }
                        } else {
                            let max = self.access_acl_refs.len();
                            if max > 0 {
                                self.access_selected_cursor =
                                    (self.access_selected_cursor + 1).min(max - 1);
                            }
                        }
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        if self.access_focus_available {
                            self.access_available_cursor =
                                self.access_available_cursor.saturating_sub(1);
                        } else {
                            self.access_selected_cursor =
                                self.access_selected_cursor.saturating_sub(1);
                        }
                    }
                    KeyCode::Left | KeyCode::Right => {
                        self.access_focus_available = !self.access_focus_available;
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if self.access_focus_available {
                            let names = self.available_acl_names();
                            if let Some(name) = names.get(self.access_available_cursor) {
                                let name = name.clone();
                                if !self.access_acl_refs.iter().any(|r| r.name == name) {
                                    self.access_acl_refs.push(AclRef {
                                        negated: false,
                                        name,
                                    });
                                }
                            }
                        } else if !self.access_acl_refs.is_empty() {
                            self.access_acl_refs.remove(self.access_selected_cursor);
                            if self.access_selected_cursor >= self.access_acl_refs.len()
                                && !self.access_acl_refs.is_empty()
                            {
                                self.access_selected_cursor = self.access_acl_refs.len() - 1;
                            }
                        }
                    }
                    KeyCode::Char('!')
                        if !self.access_focus_available
                            && self.access_selected_cursor < self.access_acl_refs.len() =>
                    {
                        self.access_acl_refs[self.access_selected_cursor].negated =
                            !self.access_acl_refs[self.access_selected_cursor].negated;
                    }
                    _ => {}
                },
                _ => {}
            },
        }
    }

    pub fn available_acl_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.config.acls.iter().map(|a| a.name.clone()).collect();
        for predef in PREDEFINED_ACLS {
            if !names.iter().any(|n| n == predef) {
                names.push((*predef).to_string());
            }
        }
        names
    }

    pub fn filtered_acl_indices(&self) -> Vec<usize> {
        if let Some(filter) = &self.acl_filter {
            let query = filter.value().to_lowercase();
            if query.is_empty() {
                return (0..self.config.acls.len()).collect();
            }
            self.config
                .acls
                .iter()
                .enumerate()
                .filter(|(_, acl)| {
                    acl.name.to_lowercase().contains(&query)
                        || acl.acl_type.to_string().to_lowercase().contains(&query)
                })
                .map(|(i, _)| i)
                .collect()
        } else {
            (0..self.config.acls.len()).collect()
        }
    }

    fn real_acl_index(&self) -> Option<usize> {
        let sel = self.acl_table_state.selected()?;
        let indices = self.filtered_acl_indices();
        indices.get(sel).copied()
    }

    fn move_selection(&mut self, delta: i32) {
        let (state, len) = match self.tab {
            Tab::Rules => {
                if self.rules_focus_acls {
                    let filtered_len = self.filtered_acl_indices().len();
                    (&mut self.acl_table_state, filtered_len)
                } else {
                    (&mut self.access_table_state, self.config.http_access.len())
                }
            }
            Tab::Direct => {
                if self.direct_focus_always {
                    (
                        &mut self.always_direct_state,
                        self.config.always_direct.len(),
                    )
                } else {
                    (&mut self.never_direct_state, self.config.never_direct.len())
                }
            }
            Tab::Auth => {
                self.auth_field = match (&self.auth_field, delta > 0) {
                    (InputField::AuthProgram, true) => InputField::AuthChildren,
                    (InputField::AuthChildren, true) => InputField::AuthRealm,
                    (InputField::AuthRealm, true) => InputField::AuthTtl,
                    (InputField::AuthTtl, true) => InputField::AuthProgram,
                    (InputField::AuthProgram, false) => InputField::AuthTtl,
                    (InputField::AuthChildren, false) => InputField::AuthProgram,
                    (InputField::AuthRealm, false) => InputField::AuthChildren,
                    (InputField::AuthTtl, false) => InputField::AuthRealm,
                    _ => InputField::AuthProgram,
                };
                return;
            }
        };

        if len == 0 {
            return;
        }

        let current = state.selected().unwrap_or(0) as i32;
        let new = (current + delta).clamp(0, len as i32 - 1) as usize;
        state.select(Some(new));

        if self.tab == Tab::Rules && self.rules_focus_acls {
            self.sync_access_to_acl();
        }
    }

    fn sync_access_to_acl(&mut self) {
        let acl_name = self
            .real_acl_index()
            .and_then(|i| self.config.acls.get(i))
            .map(|a| a.name.as_str());

        if let Some(name) = acl_name {
            let pos = self
                .config
                .http_access
                .iter()
                .position(|rule| rule.acl_refs.iter().any(|r| r.name == name));
            if let Some(idx) = pos {
                self.access_table_state.select(Some(idx));
            }
        }
    }

    fn has_selection(&self) -> bool {
        match self.tab {
            Tab::Rules => {
                if self.rules_focus_acls {
                    self.acl_table_state.selected().is_some() && !self.config.acls.is_empty()
                } else {
                    self.access_table_state.selected().is_some()
                        && !self.config.http_access.is_empty()
                }
            }
            Tab::Direct => {
                if self.direct_focus_always {
                    self.always_direct_state.selected().is_some()
                        && !self.config.always_direct.is_empty()
                } else {
                    self.never_direct_state.selected().is_some()
                        && !self.config.never_direct.is_empty()
                }
            }
            Tab::Auth => false,
        }
    }

    fn start_add(&mut self) {
        match self.tab {
            Tab::Rules => {
                if self.rules_focus_acls {
                    self.edit_name.clear();
                    self.edit_type_index = 0;
                    self.edit_values.clear();
                    self.edit_case_insensitive = false;
                    self.edit_field = InputField::Name;
                    self.screen = Screen::AclEdit { index: None };
                } else {
                    self.access_action = AccessAction::Allow;
                    self.access_acl_refs.clear();
                    self.access_available_cursor = 0;
                    self.access_selected_cursor = 0;
                    self.access_focus_available = true;
                    self.edit_field = InputField::Action;
                    self.screen = Screen::AccessEdit { index: None };
                }
            }
            Tab::Direct => {
                self.access_action = AccessAction::Allow;
                self.access_acl_refs.clear();
                self.access_available_cursor = 0;
                self.access_selected_cursor = 0;
                self.access_focus_available = true;
                self.edit_field = InputField::Action;
                self.screen = Screen::DirectEdit {
                    always: self.direct_focus_always,
                    index: None,
                };
            }
            Tab::Auth => {}
        }
    }

    fn start_edit(&mut self) {
        match self.tab {
            Tab::Rules => {
                if self.rules_focus_acls {
                    if let Some(idx) = self.real_acl_index()
                        && let Some(acl) = self.config.acls.get(idx)
                    {
                        self.edit_name.set(acl.name.clone());
                        self.edit_type_index = AclType::ALL
                            .iter()
                            .position(|t| t == &acl.acl_type)
                            .unwrap_or(0);
                        self.edit_values.set(acl.values.join("\n"));
                        self.edit_case_insensitive = acl.case_insensitive;
                        self.edit_field = InputField::Name;
                        self.screen = Screen::AclEdit { index: Some(idx) };
                    }
                } else if let Some(idx) = self.access_table_state.selected()
                    && let Some(rule) = self.config.http_access.get(idx)
                {
                    self.access_action = rule.action.clone();
                    self.access_acl_refs = rule.acl_refs.clone();
                    self.access_available_cursor = 0;
                    self.access_selected_cursor = 0;
                    self.access_focus_available = true;
                    self.edit_field = InputField::Action;
                    self.screen = Screen::AccessEdit { index: Some(idx) };
                }
            }
            Tab::Direct => {
                let (rules, state) = if self.direct_focus_always {
                    (&self.config.always_direct, &self.always_direct_state)
                } else {
                    (&self.config.never_direct, &self.never_direct_state)
                };
                if let Some(idx) = state.selected()
                    && let Some(rule) = rules.get(idx)
                {
                    self.access_action = rule.action.clone();
                    self.access_acl_refs = rule.acl_refs.clone();
                    self.access_available_cursor = 0;
                    self.access_selected_cursor = 0;
                    self.access_focus_available = true;
                    self.edit_field = InputField::Action;
                    self.screen = Screen::DirectEdit {
                        always: self.direct_focus_always,
                        index: Some(idx),
                    };
                }
            }
            Tab::Auth => {}
        }
    }

    fn do_delete(&mut self) {
        self.snapshot();
        match self.tab {
            Tab::Rules => {
                if self.rules_focus_acls {
                    if let Some(idx) = self.real_acl_index()
                        && idx < self.config.acls.len()
                    {
                        self.config.acls.remove(idx);
                        self.dirty = true;
                        let filtered_len = self.filtered_acl_indices().len();
                        if filtered_len == 0 {
                            self.acl_table_state.select(None);
                        } else if let Some(sel) = self.acl_table_state.selected()
                            && sel >= filtered_len
                        {
                            self.acl_table_state.select(Some(filtered_len - 1));
                        }
                        self.set_status("ACL deleted");
                    }
                } else if let Some(idx) = self.access_table_state.selected()
                    && idx < self.config.http_access.len()
                {
                    self.config.http_access.remove(idx);
                    self.dirty = true;
                    self.fix_selection(
                        &mut self.access_table_state.clone(),
                        self.config.http_access.len(),
                    );
                    self.set_status("Access rule deleted");
                }
            }
            Tab::Direct => {
                if self.direct_focus_always {
                    if let Some(idx) = self.always_direct_state.selected()
                        && idx < self.config.always_direct.len()
                    {
                        self.config.always_direct.remove(idx);
                        self.dirty = true;
                        self.set_status("always_direct rule deleted");
                    }
                } else if let Some(idx) = self.never_direct_state.selected()
                    && idx < self.config.never_direct.len()
                {
                    self.config.never_direct.remove(idx);
                    self.dirty = true;
                    self.set_status("never_direct rule deleted");
                }
            }
            Tab::Auth => {}
        }
    }

    fn fix_selection(&mut self, _state: &mut TableState, len: usize) {
        let state = if self.rules_focus_acls {
            &mut self.acl_table_state
        } else {
            &mut self.access_table_state
        };
        if len == 0 {
            state.select(None);
        } else if let Some(sel) = state.selected()
            && sel >= len
        {
            state.select(Some(len - 1));
        }
    }

    fn move_rule(&mut self, delta: i32) {
        self.snapshot();
        match self.tab {
            Tab::Rules if !self.rules_focus_acls => {
                if let Some(idx) = self.access_table_state.selected() {
                    let new_idx = (idx as i32 + delta)
                        .clamp(0, self.config.http_access.len() as i32 - 1)
                        as usize;
                    if new_idx != idx {
                        self.config.http_access.swap(idx, new_idx);
                        self.access_table_state.select(Some(new_idx));
                        self.dirty = true;
                    }
                }
            }
            Tab::Direct => {
                if self.direct_focus_always {
                    if let Some(idx) = self.always_direct_state.selected() {
                        let len = self.config.always_direct.len();
                        let new_idx = (idx as i32 + delta).clamp(0, len as i32 - 1) as usize;
                        if new_idx != idx {
                            self.config.always_direct.swap(idx, new_idx);
                            self.always_direct_state.select(Some(new_idx));
                            self.dirty = true;
                        }
                    }
                } else if let Some(idx) = self.never_direct_state.selected() {
                    let len = self.config.never_direct.len();
                    let new_idx = (idx as i32 + delta).clamp(0, len as i32 - 1) as usize;
                    if new_idx != idx {
                        self.config.never_direct.swap(idx, new_idx);
                        self.never_direct_state.select(Some(new_idx));
                        self.dirty = true;
                    }
                }
            }
            _ => {}
        }
    }

    fn save_acl_edit(&mut self) {
        let name = self.edit_name.value().trim().to_string();
        if name.is_empty() {
            self.set_status("ACL name cannot be empty");
            return;
        }

        let acl_type = AclType::ALL[self.edit_type_index].clone();
        let values: Vec<String> = self
            .edit_values
            .value()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        if values.is_empty() {
            self.set_status("ACL must have at least one value");
            return;
        }

        for val in &values {
            if let Err(e) = crate::validate::validate_acl_value(&acl_type, val) {
                self.set_status(format!("Validation: {e}"));
                return;
            }
        }

        let acl = Acl {
            name,
            acl_type,
            case_insensitive: self.edit_case_insensitive,
            values,
        };

        self.snapshot();
        if let Screen::AclEdit { index: Some(idx) } = self.screen {
            self.config.acls[idx] = acl;
            self.set_status("ACL updated");
        } else {
            self.config.acls.push(acl);
            let len = self.config.acls.len();
            self.acl_table_state.select(Some(len - 1));
            self.set_status("ACL added");
        }

        self.dirty = true;
        self.screen = Screen::List;
    }

    fn save_access_edit(&mut self) {
        if self.access_acl_refs.is_empty() {
            self.set_status("Rule must reference at least one ACL");
            return;
        }

        self.snapshot();
        match self.screen.clone() {
            Screen::AccessEdit { index } => {
                let rule = AccessRule {
                    action: self.access_action.clone(),
                    acl_refs: self.access_acl_refs.clone(),
                };
                if let Some(idx) = index {
                    self.config.http_access[idx] = rule;
                    self.set_status("Access rule updated");
                } else {
                    self.config.http_access.push(rule);
                    let len = self.config.http_access.len();
                    self.access_table_state.select(Some(len - 1));
                    self.set_status("Access rule added");
                }
            }
            Screen::DirectEdit { always, index } => {
                let rule = DirectRule {
                    action: self.access_action.clone(),
                    acl_refs: self.access_acl_refs.clone(),
                };
                if always {
                    if let Some(idx) = index {
                        self.config.always_direct[idx] = rule;
                    } else {
                        self.config.always_direct.push(rule);
                        let len = self.config.always_direct.len();
                        self.always_direct_state.select(Some(len - 1));
                    }
                    self.set_status("always_direct rule saved");
                } else {
                    if let Some(idx) = index {
                        self.config.never_direct[idx] = rule;
                    } else {
                        self.config.never_direct.push(rule);
                        let len = self.config.never_direct.len();
                        self.never_direct_state.select(Some(len - 1));
                    }
                    self.set_status("never_direct rule saved");
                }
            }
            _ => {}
        }

        self.dirty = true;
        self.screen = Screen::List;
    }

    pub fn save_auth(&mut self) {
        fn opt(s: &str) -> Option<String> {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        }

        self.snapshot();
        self.config.auth_param.program = opt(self.auth_program.value());
        self.config.auth_param.children = opt(self.auth_children.value());
        self.config.auth_param.realm = opt(self.auth_realm.value());
        self.config.auth_param.credentialsttl = opt(self.auth_ttl.value());
        self.dirty = true;
        self.set_status("Auth configuration saved");
    }

    fn save_config(&mut self) {
        if self.file_path.exists() {
            let backup = self.file_path.with_extension("conf.bak");
            if let Err(e) = std::fs::copy(&self.file_path, &backup) {
                self.set_status(format!("Backup failed: {e}"));
                return;
            }
        }

        let content = crate::writer::write_config(&self.config);
        match std::fs::write(&self.file_path, &content) {
            Ok(()) => {
                self.dirty = false;
                self.set_status(format!("Saved to {}", self.file_path.display()));
            }
            Err(e) => {
                self.set_status(format!("Error saving: {e}"));
            }
        }
    }

    pub fn handle_auth_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::F(9) {
            self.menu_active = !self.menu_active;
            self.menu_index = 0;
            self.menu_item = 0;
            return;
        }

        if self.menu_active {
            self.handle_menu_key(key);
            return;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('q') => {
                    if self.dirty {
                        self.screen = Screen::ConfirmQuit;
                    } else {
                        self.should_quit = true;
                    }
                    return;
                }
                KeyCode::Char('s') => {
                    self.save_config();
                    return;
                }
                KeyCode::Char('z') => {
                    self.undo();
                    return;
                }
                KeyCode::Char('y') => {
                    self.redo();
                    return;
                }
                _ => {}
            }
        }

        match key.code {
            KeyCode::Char('?') | KeyCode::F(1) => self.help_visible = true,
            KeyCode::Esc => self.tab = self.tab.next(),
            KeyCode::BackTab => self.tab = self.tab.prev(),
            KeyCode::Tab => {
                self.auth_field = match self.auth_field {
                    InputField::AuthProgram => InputField::AuthChildren,
                    InputField::AuthChildren => InputField::AuthRealm,
                    InputField::AuthRealm => InputField::AuthTtl,
                    InputField::AuthTtl => InputField::AuthProgram,
                    _ => InputField::AuthProgram,
                };
            }
            KeyCode::F(2) => {
                self.save_auth();
            }
            _ => {
                let field = match self.auth_field {
                    InputField::AuthProgram => &mut self.auth_program,
                    InputField::AuthChildren => &mut self.auth_children,
                    InputField::AuthRealm => &mut self.auth_realm,
                    InputField::AuthTtl => &mut self.auth_ttl,
                    _ => return,
                };
                field.handle_key(key);
            }
        }
    }
}
