use crate::app::App;

impl App {
    pub fn switch_editor_view(&mut self) {
        self.editor_view.next();
    }
}
