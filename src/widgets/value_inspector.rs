use crate::app::state::FocusedPane;
use crate::app::state::State;

use hxdmp::hexdump;

use notatin::cell_value::CellValue;
use ratatui::prelude::Alignment;
use ratatui::text::Text;
use ratatui::widgets::StatefulWidget;
use ratatui::{
    style::*,
    symbols::border,
    widgets::{block::*, *},
};

pub struct ValueInspector {}

impl StatefulWidget for &mut ValueInspector {
    type State = State;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let title = Title::from("Value Inspector".to_string());

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(match state.focused_pane {
                FocusedPane::ValueInspector => border::THICK,
                _ => border::PLAIN,
            })
            .border_style(match state.focused_pane {
                FocusedPane::ValueInspector => Color::Green,
                _ => Color::default(),
            });

        let content: Text = match state.navigation.selected_value {
            Some(ref value) => Text::from(format!(
                "Data Type: {:?}\nValue Data: {}",
                value.data_type,
                match value.get_content().0 {
                    CellValue::Binary(blob) => {
                        let mut dump: Vec<u8> = Vec::new();
                        let _ = hexdump(&blob, &mut dump);
                        String::from_utf8(dump).unwrap()
                    }
                    _ => format!("{:?}", value.get_content().0),
                }
            )),
            None => Text::from(""),
        };

        let display = Paragraph::new(content).block(block);

        <Paragraph as Widget>::render(display, area, buf);
    }
}
