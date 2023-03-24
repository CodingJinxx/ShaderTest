#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tool {
    None,
    Select,
    BuildWall,
    PlaceLight,
    Delete
}