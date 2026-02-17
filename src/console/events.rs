#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiEvent {
    InputChanged,
    CommandSubmitted,
    SelectionChanged,
    TabChanged,
    RefreshTick,
    BackRequested,
}
