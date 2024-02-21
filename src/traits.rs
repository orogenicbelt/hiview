use std::io;

trait KeyEventHandler {
    fn handle_key_events(&mut self) -> io::Result<()>;
}
