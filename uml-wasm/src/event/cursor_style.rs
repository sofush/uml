#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum CursorStyle {
    #[default]
    Default,
    Grab,
    Grabbing,
}
