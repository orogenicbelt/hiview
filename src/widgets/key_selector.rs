use crate::app::navigation::KeySort;
use crate::app::state::{FocusedPane, State};
use notatin::cell_key_node::CellKeyNode;
use ratatui::layout::Constraint;
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::StatefulWidget;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::*,
    symbols::border,
    text::Line,
    widgets::{block::*, *},
};
use std::convert::Into;

use ratatui::prelude::Alignment;

pub struct KeySelector;

pub struct UIKey<'cell> {
    key: &'cell CellKeyNode,
    styles: [Style; 3],
}

pub struct UIKeySet<'cell> {
    keys: Vec<&'cell CellKeyNode>,
    sort_column: KeySort,
    focused: bool,
}

fn make_table_block(focused: bool) -> Block<'static> {
    let title = Title::from("subkeys".to_string());
    let instructions = Title::from(Line::from(vec![
        " Enter Subkey ".into(),
        "<L>".blue().bold(),
        " Go to parent key ".into(),
        "<H>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]));
    Block::default()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(match focused {
            true => border::THICK,
            false => border::PLAIN,
        })
        .border_style(match focused {
            true => Color::Green,
            false => Color::default(),
        })
}

impl From<UIKeySet<'_>> for Table<'_> {
    fn from(value: UIKeySet<'_>) -> Self {
        let mut styles = [Style::default(); 3];
        styles[value.sort_column as usize].bg = Some(Color::DarkGray);

        let rows = value
            .keys
            .iter()
            .map(|key| UIKey { key, styles }.into())
            .collect::<Vec<Row>>();

        let widest_count = value
            .keys
            .iter()
            .map(|key| key.detail.number_of_sub_keys().to_string().len())
            .max()
            .unwrap_or(0);

        let widest_key = value
            .keys
            .iter()
            .map(|key| key.key_name.len())
            .max()
            .unwrap_or(0);

        Table::new(
            rows,
            vec![
                Constraint::Length(widest_count as u16),
                Constraint::Fill(widest_key as u16),
                Constraint::Length(24),
            ],
        )
        .block(make_table_block(value.focused))
        .highlight_style(Style::new().add_modifier(Modifier::BOLD))
        .highlight_symbol(Text::from("|").blue())
    }
}

impl From<UIKey<'_>> for Row<'_> {
    fn from(val: UIKey<'_>) -> Self {
        Row::new([
            Cell::new(format!("{}", val.key.detail.number_of_sub_keys())).style(val.styles[0]),
            Cell::new(val.key.key_name.clone()).style(val.styles[1]),
            Cell::new(
                Text::from(
                    val.key
                        .last_key_written_date_and_time()
                        .to_utc()
                        .to_string(),
                )
                .alignment(Alignment::Right),
            )
            .style(val.styles[2]),
        ])
    }
}

impl StatefulWidget for &mut KeySelector {
    type State = State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut State)
    where
        Self: Sized,
    {
        let table = Table::from(UIKeySet {
            keys: state.navigation.current_subkeys.iter().collect(),
            focused: state.focused_pane == FocusedPane::KeySelector,
            sort_column: state.navigation.key_sort_method,
        });

        <Table as StatefulWidget>::render(
            table,
            area,
            buf,
            &mut state.navigation.table_states.key_selector_state,
        );
    }
}
