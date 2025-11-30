use crate::app::App;

impl App {
    pub fn switch_editor_mode(&mut self) {
        self.editor_view.next();
    }
}
