// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-blockly: blockly component model.
//!
//! Real implementation: blockly_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlocklyItem {
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
pub enum BlocklyError {
    EmptyBlocklyItemName,
    DuplicateBlocklyItem(String),
    EmptyEventSubject,
}

impl fmt::Display for BlocklyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBlocklyItemName => write!(f, "blockly_item name must not be empty"),
            Self::DuplicateBlocklyItem(id) => write!(f, "duplicate blockly_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for BlocklyError {}

pub struct Blockly {
    pub name: String,
    blockly_items: BTreeMap<String, BlocklyItem>,
    events: BTreeMap<String, Event>,
}

impl Blockly {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            blockly_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_blockly_item(&mut self, item: BlocklyItem) -> Result<(), BlocklyError> {
        if item.name.trim().is_empty() {
            return Err(BlocklyError::EmptyBlocklyItemName);
        }
        let id = item.id.clone();
        if self.blockly_items.contains_key(&id) {
            return Err(BlocklyError::DuplicateBlocklyItem(id.clone()));
        }
        self.blockly_items.insert(id, item);
        Ok(())
    }

    pub fn blockly_item_count(&self) -> usize {
        self.blockly_items.len()
    }

    pub fn get_blockly_item(&self, id: &str) -> Option<&BlocklyItem> {
        self.blockly_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), BlocklyError> {
        if event.subject.trim().is_empty() {
            return Err(BlocklyError::EmptyEventSubject);
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

    fn sample_blockly_item(id: &str, name: &str) -> BlocklyItem {
        BlocklyItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Blockly::new("Blockly");
        assert_eq!(m.name, "Blockly");
        assert_eq!(m.blockly_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_blockly_item() {
        let mut m = Blockly::new("Blockly");
        m.add_blockly_item(sample_blockly_item("a1", "Alpha")).unwrap();
        assert_eq!(m.blockly_item_count(), 1);
        assert!(m.get_blockly_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Blockly::new("Blockly");
        assert!(m.add_blockly_item(sample_blockly_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Blockly::new("Blockly");
        m.add_blockly_item(sample_blockly_item("x", "One")).unwrap();
        assert!(m.add_blockly_item(sample_blockly_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Blockly::new("Blockly");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
