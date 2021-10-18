use crate::datatype::Number;

pub struct FilePosition {
    url: String,
    file: Option<String>,
    line: Number,
    column: Number,
    source: Option<String>,
}

/// Represents the source CSS.
pub struct Input {
    css: String,
    map:
}
