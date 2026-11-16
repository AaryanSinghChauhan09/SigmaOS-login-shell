// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-login-shell: login shell component model.
//!
//! Real implementation: shells, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shell {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginShellError {
    EmptyShellName,
    DuplicateShell(String),
    EmptyEventSubject,
}

impl fmt::Display for LoginShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyShellName => write!(f, "shell name must not be empty"),
            Self::DuplicateShell(id) => write!(f, "duplicate shell {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LoginShellError {}

pub struct LoginShell {
    pub name: String,
    shells: BTreeMap<String, Shell>,
    events: BTreeMap<String, Event>,
}

impl LoginShell {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            shells: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_shell(&mut self, item: Shell) -> Result<(), LoginShellError> {
        if item.name.trim().is_empty() {
            return Err(LoginShellError::EmptyShellName);
        }
        let id = item.id.clone();
        if self.shells.contains_key(&id) {
            return Err(LoginShellError::DuplicateShell(id.clone()));
        }
        self.shells.insert(id, item);
        Ok(())
    }

    pub fn shell_count(&self) -> usize {
        self.shells.len()
    }

    pub fn get_shell(&self, id: &str) -> Option<&Shell> {
        self.shells.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LoginShellError> {
        if event.subject.trim().is_empty() {
            return Err(LoginShellError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_shell(id: &str, name: &str) -> Shell {
        Shell {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = LoginShell::new("LoginShell");
        assert_eq!(m.name, "LoginShell");
        assert_eq!(m.shell_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_shell() {
        let mut m = LoginShell::new("LoginShell");
        m.add_shell(sample_shell("a1", "Alpha")).unwrap();
        assert_eq!(m.shell_count(), 1);
        assert!(m.get_shell("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = LoginShell::new("LoginShell");
        assert!(m.add_shell(sample_shell("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = LoginShell::new("LoginShell");
        m.add_shell(sample_shell("x", "One")).unwrap();
        assert!(m.add_shell(sample_shell("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = LoginShell::new("LoginShell");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
