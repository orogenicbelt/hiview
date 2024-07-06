use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::StatefulWidget,
};

use crate::app::state::State;

use super::{
    key_selector::KeySelector, value_inspector::ValueInspector, value_selector::ValueSelector,
};

pub struct MainWidget {}

impl StatefulWidget for MainWidget {
    type State = State;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut State,
    ) where
        Self: Sized,
    {
        let main_layout = Layout::new(
            Direction::Horizontal,
            vec![Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .vertical_margin(2);

        let value_layout = Layout::new(
            Direction::Vertical,
            vec![Constraint::Percentage(40), Constraint::Percentage(60)],
        )
        .horizontal_margin(2);

        let main_rects = main_layout.split(area);
        let value_rects = value_layout.split(main_rects[1]);

        let mut key_selector = KeySelector {};
        let mut value_selector = ValueSelector {};
        let mut value_inspector = ValueInspector {};

        key_selector.render(main_rects[0], buf, state);
        value_selector.render(value_rects[0], buf, state);
        value_inspector.render(value_rects[1], buf, state);
    }
}
