use std::num::NonZero;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use lru::LruCache;
use notatin::cell_key_node::CellKeyNode;
use notatin::cell_key_value::CellKeyValue;
use notatin::parser::Parser;
use ratatui::widgets::TableState;

/// Contains and manages information about where we are currently
/// located within the registry hive tree structure.
#[derive(Debug)]
pub struct Navigation {
    pub parser: Parser,
    pub current_key: CellKeyNode,
    pub selected_subkey: Option<CellKeyNode>,
    pub current_subkeys: Vec<CellKeyNode>,
    pub current_values: Vec<CellKeyValue>,
    pub selected_value: Option<CellKeyValue>,
    pub table_states: CurrentKeyState,
    pub key_state_cache: LruCache<usize, TableState>,
    pub value_state_cache: LruCache<usize, TableState>,
    pub key_sort_method: KeySort,
}

#[derive(Copy, Clone, Default, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeySort {
    #[default]
    SubkeyCount = 0,
    KeyName = 1,
    LastWrite = 2,
}

impl Navigation {
    pub fn new(mut parser: Parser) -> Self {
        let current_key = parser
            .get_root_key()
            .expect("This hive to have a root key")
            .expect("The root key to not be None");

        Navigation {
            parser,
            current_key: current_key.clone(),
            table_states: CurrentKeyState::default(),
            selected_subkey: None,
            current_subkeys: vec![],
            current_values: vec![],
            selected_value: None,
            key_state_cache: LruCache::new(NonZero::new(200).unwrap()),
            value_state_cache: LruCache::new(NonZero::new(200).unwrap()),
            key_sort_method: KeySort::default(),
        }
        .with_selected_key(current_key.clone())
    }

    pub fn with_selected_key(mut self, key: CellKeyNode) -> Self {
        self.select_key(key);
        self
    }

    pub fn sort_subkeys(&mut self, sort_type: KeySort) {
        match sort_type {
            KeySort::LastWrite => self
                .current_subkeys
                .sort_by_key(|key| key.last_key_written_date_and_time()),
            KeySort::KeyName => self.current_subkeys.sort_by_key(|key| key.key_name.clone()),
            KeySort::SubkeyCount => self
                .current_subkeys
                .sort_by_key(|key| key.detail.number_of_sub_keys()),
        }
        if let Some(ref sk) = self.selected_subkey {
            let index = self.current_subkeys.iter().position(|key| *sk == *key);
            self.table_states.key_selector_state.select(index)
        }
        self.key_sort_method = sort_type;
    }

    pub fn change_subkey_sort(&mut self) {
        self.sort_subkeys(
            KeySort::iter()
                .cycle()
                .skip_while(|sort| *sort != self.key_sort_method)
                .nth(1)
                .expect("There to be a continuous iterator"),
        )
    }

    pub fn select_key(&mut self, key: CellKeyNode) {
        // Save this key/value selection states in LRU caches in case we navigate back to this point
        // Only do this if the key/value selection is in a non-default state so that we don't
        // needlessly fill the LRU cache with default states.
        if self.table_states.key_selector_state.selected().unwrap_or(0) != 0 {
            self.key_state_cache.put(
                self.current_key.file_offset_absolute,
                self.table_states.key_selector_state.clone(),
            );
        }
        if self
            .table_states
            .value_selector_state
            .selected()
            .unwrap_or(0)
            != 0
        {
            self.value_state_cache.put(
                self.current_key.file_offset_absolute,
                self.table_states.value_selector_state.clone(),
            );
        }

        // Update the current members
        self.current_key = key;
        self.current_subkeys = self.current_key.read_sub_keys(&mut self.parser);
        self.current_values = self.current_key.value_iter().collect::<Vec<CellKeyValue>>();

        // Get the saved table states for this key, or initialize new ones if they don't exist
        self.table_states.key_selector_state = self
            .key_state_cache
            .get(&self.current_key.file_offset_absolute)
            .unwrap_or(&TableState::default().with_selected(0))
            .clone();

        // Select the current subkey + value
        self.select_subkey(
            self.current_subkeys
                .get(self.table_states.key_selector_state.selected().unwrap_or(0))
                .cloned(),
        );
    }

    pub fn enter_key(&mut self) {
        if let Some(subkey) = &self.selected_subkey {
            self.select_key(subkey.clone());
        }
    }

    pub fn leave_key(&mut self) {
        if let Ok(Some(parent_key)) = self.parser.get_parent_key(&mut self.current_key) {
            self.select_key(parent_key.clone());
        }
    }

    pub fn select_subkey(&mut self, key: Option<CellKeyNode>) {
        if let Some(index) = self.table_states.value_selector_state.selected() {
            if index != 0 {
                self.value_state_cache.put(
                    self.selected_subkey.as_ref().unwrap().file_offset_absolute,
                    self.table_states.value_selector_state.to_owned(),
                );
            }
        }

        self.selected_subkey = key;

        match self.selected_subkey {
            Some(ref sk) => {
                self.current_values = sk.value_iter().collect::<Vec<CellKeyValue>>();

                self.table_states.value_selector_state = self
                    .value_state_cache
                    .get(&sk.file_offset_absolute)
                    .unwrap_or(&TableState::default())
                    .clone();
                if let Some(index) = self.table_states.value_selector_state.selected() {
                    self.selected_value = Some(self.current_values[index].clone());
                } else if !self.current_values.is_empty() {
                    self.selected_value = Some(self.current_values[0].clone());
                    self.table_states.value_selector_state.select(Some(0));
                }
            }
            None => {
                self.current_values = vec![];
                self.table_states.value_selector_state = TableState::new();
                self.selected_value = None;
            }
        }
    }

    /// Navigates up or down the subkey list by `n_keys` indices.
    /// Updates the `current_subkey` and `current_values` members,
    /// and the table state for the current values,
    pub fn change_subkey_by(&mut self, n_keys: isize) {
        let index = self.table_states.key_selector_state.selected().unwrap_or(0);

        let new_index = std::cmp::min(
            std::cmp::max(0, index as isize + n_keys) as usize,
            self.current_key
                .cell_sub_key_offsets_absolute
                .len()
                .saturating_sub(1),
        );

        self.table_states.key_selector_state.select(Some(new_index));

        self.select_subkey(self.current_subkeys.get(new_index).cloned());
    }

    pub fn change_value_by(&mut self, n_keys: isize) {
        let index = self
            .table_states
            .value_selector_state
            .selected()
            .unwrap_or(0);

        if let Some(subkey) = &self.selected_subkey {
            let new_index = std::cmp::min(
                std::cmp::max(0, index as isize + n_keys) as usize,
                subkey.value_iter().count().saturating_sub(1),
            );
            self.table_states
                .value_selector_state
                .select(Some(new_index));
            self.selected_value = self.current_values.get(new_index).cloned();
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct CurrentKeyState {
    pub key_selector_state: TableState,
    pub value_selector_state: TableState,
}
