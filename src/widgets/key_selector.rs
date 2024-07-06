use crate::app::state::{State, FocusedPane};
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

use ratatui::prelude::Alignment;

pub struct KeySelector;

impl StatefulWidget for &mut KeySelector {
    type State = State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut State)
    where
        Self: Sized,
    {
        let title = Title::from("subkeys".to_string());
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
            .border_set(match state.focused_pane {
                FocusedPane::KeySelector => border::THICK,
                _ => border::PLAIN,
            })
            .border_style(match state.focused_pane {
                FocusedPane::KeySelector => Color::Green,
                _ => Color::default(),
            });

        let rows: Vec<Row> = state
            .navigation
            .current_key
            .read_sub_keys(&mut state.navigation.parser)
            .iter()
            .map(|key| Row::new(vec![Cell::new(key.key_name.clone())]))
            .collect::<Vec<Row>>();

        let table = Table::new(rows, vec![80])
            .block(block)
            .highlight_style(Style::new().add_modifier(Modifier::BOLD))
            .highlight_symbol(Text::from("|").blue());

        <Table as StatefulWidget>::render(
            table,
            area,
            buf,
            &mut state.navigation.table_states.key_selector_state,
        );
    }
}
