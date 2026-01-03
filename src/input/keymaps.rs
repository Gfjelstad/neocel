use std::collections::HashMap;

use crate::{commands::Key, input::Token};

#[derive(Clone)]
pub struct ActionNode {
    pub children: HashMap<Key, ActionNode>,
    pub action: Option<Token>,
}

pub trait KeymapProvider {
    fn define_keymap(&self) -> ActionNode;

    // Required to access the implementer's cached field
    fn get_keymap_cache(&self) -> &Option<ActionNode>;
    fn set_keymap_cache(&mut self, value: Option<ActionNode>);

    // Default method with caching logic
    fn keymap(&mut self) -> &ActionNode {
        if self.get_keymap_cache().is_none() {
            let node = self.define_keymap();
            self.set_keymap_cache(Some(node));
        }
        self.get_keymap_cache().as_ref().unwrap()
    }
}
