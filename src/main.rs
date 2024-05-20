use std::io;

use hiview::key_selector::RegistryKeySelectorWidget;
use hiview::tui;

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let path = args
        .get(1)
        .expect("Must supply path to registry hive")
        .clone();

    let mut terminal = tui::init()?;
    let app_result = RegistryKeySelectorWidget::new(&path).run(&mut terminal);

    tui::restore()?;
    app_result
}
