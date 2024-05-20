use std::borrow::BorrowMut;
use crate::tui;
use std::{cmp, io};
use std::num::NonZeroUsize;
use lru::LruCache;
use notatin::{cell_key_node::CellKeyNode, parser::Parser, parser_builder::ParserBuilder};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    symbols::border,
    widgets::{block::*, *},
};

use ratatui::prelude::*;

use crate::app_state::{Level, TableType};

pub struct WrappedCellKeyNode(CellKeyNode);


impl Into<Text<'static>> for WrappedCellKeyNode {
    fn into(self) -> Text<'static> {
        Text::from(self.0.key_name)
    }
}

#[derive(Debug)]
pub struct RegistryKeySelectorWidget {
    parent_levels: Vec<Level>,
    state_cache: LruCache<(TableType, usize), TableState>,
    current_level: Level,
    parser: Parser,
    exit: bool,
}

impl RegistryKeySelectorWidget {
    pub fn new(path: &str) -> Self {
        let hive = ParserBuilder::from_path(path.to_string());
        let mut parser = hive.build().expect("The hive did not parse correctly");
        let mut root = parser.get_root_key().unwrap().unwrap();
        RegistryKeySelectorWidget {
            parent_levels: vec![],
            state_cache: LruCache::new(NonZeroUsize::new(200).unwrap()),
            current_level: Level {
                keys: root.read_sub_keys(&mut parser),
                key_file_offset: root.file_offset_absolute,
                table_state: TableState::new().with_selected(0),
            },
            parser,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let mut state = self.current_level.table_state.clone();
        frame.render_stateful_widget(self, frame.size(), &mut state);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn enter_key(&mut self) {
        if let Some(index) = self.current_level.table_state.selected() {
            let mut new_parent_key = self.current_level.keys[index].clone();
            let parser = self.parser.borrow_mut();
            let next_keys = new_parent_key.read_sub_keys(parser);
            if next_keys.len() == 0 {
                return;
            }
            let cur = self.current_level.clone();
            self.parent_levels.push(cur);
            self.current_level = Level {
                keys: next_keys,
                key_file_offset: new_parent_key.file_offset_absolute,
                table_state: self
                    .state_cache
                    .pop(&(TableType::RegistryKey, new_parent_key.file_offset_absolute))
                    .unwrap_or(TableState::new().with_offset(0)),
            };
        }
    }

    fn leave_key(&mut self) {
        match self.parent_levels.len() {
            0 => {}
            _ => {
                let parent = self.parent_levels.pop().unwrap();
                self.state_cache.push(
                    (TableType::RegistryKey, self.current_level.key_file_offset),
                    self.current_level.table_state.clone(),
                );
                self.current_level = parent;
            }
        }
    }

    fn previous_key(&mut self) {
        match self.current_level.table_state.selected() {
            Some(index) => self.current_level.table_state.select(index.checked_sub(1)),
            None => self.current_level.table_state.select(Some(0)),
        }
    }

    fn next_key(&mut self) {
        match self.current_level.table_state.selected() {
            Some(index) => {
                let n = self.current_level.keys.len();
                self.current_level
                    .table_state
                    .select(Some(cmp::min(index + 1, n - 1)))
            }
            None => self.current_level.table_state.select(Some(0)),
        }
    }

    fn last_key(&mut self) {
        let n = self.current_level.keys.len();
        self.current_level
            .table_state
            .select(Some(n-1))
    }

    fn first_key(&mut self) {
        self.current_level
            .table_state
            .select(Some(0))
    }

    fn quit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('g') => self.first_key(),
            KeyCode::Char('G') => self.last_key(),
            KeyCode::Char('h') => self.leave_key(),
            KeyCode::Char('l') => self.enter_key(),
            KeyCode::Char('j') => self.next_key(),
            KeyCode::Char('k') => self.previous_key(),
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }

    fn current_path(&self) -> Option<String> {
        Some(self.parent_levels.last()?.path()?)
    }
}

impl StatefulWidget for &mut RegistryKeySelectorWidget {
    type State = TableState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut TableState)
    where
        Self: Sized,
    {
        let title = Title::from(self.current_path().unwrap_or("b-hive".to_string()).bold());
        let instructions = Title::from(Line::from(vec![
            " Enter Subkey ".into(),
            "<L>".blue().bold(),
            " Go to parent key ".into(),
            "<H>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let rows: Vec<Row> = self
            .current_level
            .keys
            .clone()
            .iter()
            .map(|sub_key| Row::new(vec![WrappedCellKeyNode(sub_key.to_owned())]))
            .collect();

        let table = Table::new(rows, vec![80])
            .block(block)
            .highlight_style(Style::new().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>");
        <Table as StatefulWidget>::render(table, area, buf, state);
    }
}

