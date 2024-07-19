use crate::app::state::FocusedPane::*;
use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Tab => {
            if key_event.modifiers == KeyModifiers::SHIFT {
                app.state.focus_previous_tab()
            } else {
                app.state.focus_next_tab()
            }
        }

        // Other handlers you could add here.
        _ => match app.state.focused_pane {
            KeySelector => handle_key_selector_key_events(key_event, app),
            ValueSelector => handle_value_selector_key_events(key_event, app),
            ValueInspector => handle_value_inspector_key_events(key_event, app),
        }?,
    }
    Ok(())
}

pub fn handle_key_selector_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('l') => app.state.navigation.enter_key(),
        KeyCode::Char('h') => app.state.navigation.leave_key(),
        KeyCode::Char('j') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.state.navigation.change_subkey_by(10)
            } else {
                app.state.navigation.change_subkey_by(1)
            }
        }
        KeyCode::Char('k') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.state.navigation.change_subkey_by(-10)
            } else {
                app.state.navigation.change_subkey_by(-1)
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_value_selector_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('j') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.state.navigation.change_value_by(10)
            } else {
                app.state.navigation.change_value_by(1)
            }
        }
        KeyCode::Char('k') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.state.navigation.change_value_by(-10)
            } else {
                app.state.navigation.change_value_by(-1)
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_value_inspector_key_events(_key_event: KeyEvent, _app: &mut App) -> AppResult<()> {
    Ok(())
}
