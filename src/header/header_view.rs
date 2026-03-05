use ratatui::widgets::ListState;

#[derive(Default, Debug)]
pub struct HeaderView {
    pub list_state: ListState,
    pub entrypoint: u32,
}
