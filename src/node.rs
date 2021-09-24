use crate::datatype::Number;

#[allow(dead_code)]
pub(crate) struct Position {
    /// Source offset in file. It starts from 0.
    offset: Number,

    /// Source line in file. In contrast to `offset` it starts from 1.
    column: Number,

    /// Source column in file.
    line: Number,
}




#[test]
fn test_hello_world() {}