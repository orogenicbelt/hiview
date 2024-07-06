use ratatui::Frame;

use crate::widgets::main::MainWidget;

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(_app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let main_widget = MainWidget{};
    frame.render_stateful_widget(main_widget, frame.size(), &mut _app.state);
}
