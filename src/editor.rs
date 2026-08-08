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
    pub fn previous(&mut self) {
        match self {
            AppView::Hex => *self = AppView::Header,
            AppView::Text => *self = AppView::Hex,
            AppView::Header => *self = AppView::Text,
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
    DialogReverseTruncate,
    DialogSearch,
    DialogStrings,
    DialogStringsRegex,
    DialogTruncate,
    Error,
    HexEditing,
    HexSelection,
    Normal,
}
