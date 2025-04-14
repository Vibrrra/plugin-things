use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DropOperation {
    None,
    Copy,
    Move,
    Link,
}

#[derive(Clone, Debug, Default)]
pub enum DropData {
    #[default]
    None,
    Files(Vec<PathBuf>),
}
