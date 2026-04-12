#[derive(PartialEq)]
pub enum AppView {
    Text,
    Hex,
}

impl AppView {
    pub fn next(&mut self) {
        match self {
            AppView::Text => *self = AppView::Hex,
            AppView::Hex => *self = AppView::Text,
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
