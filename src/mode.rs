#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Normal,
    Search,
    BookMark,
    Filter,
    Highlight,
}
