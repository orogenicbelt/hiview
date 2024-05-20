use notatin::cell_key_node::CellKeyNode;
use ratatui::widgets::{TableState, Widget};
#[derive(Debug, Clone)]


#[derive(PartialEq, Eq, Hash)]
pub enum TableType {
    RegistryKey,
    RegistryValue,
}

#[derive(Debug, Clone)]
pub struct Level {
    pub table_state: TableState,
    pub key_file_offset: usize,
    pub keys: Vec<CellKeyNode>,
}

impl Level {
    pub fn path(&self) -> Option<String> {
        let idx = self.table_state.selected()?;
        let key = &self.keys[idx];
        Some(key.path.clone())
    }
}

pub struct AppState<'a> {
    _selected_widget: &'a dyn Widget,
}

impl<'a> AppState<'_> {
}
