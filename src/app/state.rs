use crate::app::navigation::Navigation;
use notatin::parser::Parser;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Default, EnumIter, PartialEq)]
pub enum FocusedPane {
    #[default]
    KeySelector,
    ValueSelector,
    ValueInspector,
}

#[derive(Debug)]
pub struct State {
    pub navigation: Navigation,
    pub focused_pane: FocusedPane,
}

impl State {
    pub fn new(parser: Parser) -> Self {
        State {
            navigation: Navigation::new(parser),
            focused_pane: FocusedPane::default(),
        }
    }

    pub fn focus_next_tab(&mut self) {
        self.focused_pane = FocusedPane::iter()
            .cycle()
            .skip_while(|x| *x != self.focused_pane)
            .nth(1)
            .unwrap();
    }

    pub fn focus_previous_tab(&mut self) {
        self.focused_pane = FocusedPane::iter()
            .rev()
            .cycle()
            .skip_while(|x| *x != self.focused_pane)
            .nth(1)
            .unwrap()
    }
}
