use std::collections::HashMap;

pub enum Node<'i> {
    Sequence(Vec<Node<'i>>),
    InlineSequence(Vec<LeafValue>),
    Map(HashMap<&'i str, Node<'i>>),
    EnumVariant(&'i str, Box<Node<'i>>),
    Fields(HashMap<&'i str, Node<'i>>),
    Leaf(LeafValue),
}

pub enum LeafValue {
    Null,
    U64(u64),
    I64(i64),
    F64(f64),
    String(String),
    Bool(bool),
}
