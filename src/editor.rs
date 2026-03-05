#[derive(PartialEq)]
pub enum AppView {
    Text,
    Hex,
    Header,
}

impl AppView {
    pub fn next(&mut self) {
        match self {
            AppView::Hex => *self = AppView::Text,
            AppView::Text => *self = AppView::Header,
            AppView::Header => *self = AppView::Hex,
        }
    }
}

#[derive(PartialEq)]
pub enum UIState {
    Command,
    DialogCalculator,
    DialogComment,
    DialogEncoding,
    DialogHelp,
    DialogLog,
    DialogNames,
    DialogNamesRegex,
    DialogSearch,
    DialogStrings,
    DialogStringsRegex,
    Error,
    HexEditing,
    HexSelection,
    Normal,
}
