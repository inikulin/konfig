use super::type_name::TypeName;
use super::PathItem;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type NodeCell<'i> = Rc<RefCell<Node<'i>>>;

#[derive(Debug, PartialEq)]
pub enum Node<'i> {
    Sequence(Vec<NodeCell<'i>>),
    Map(HashMap<String, NodeCell<'i>>),
    NewTypeEnumVariant(&'i str, NodeCell<'i>),
    Fields(HashMap<&'i str, NodeCell<'i>>),
    Leaf(Leaf<'i>),
}

impl Node<'_> {
    pub(super) fn is_multitenant(&self) -> bool {
        matches!(self, Node::Sequence(_) | Node::Map(_) | Node::Fields(_))
    }
}

impl<'i> From<Node<'i>> for NodeCell<'i> {
    fn from(node: Node<'i>) -> Self {
        Rc::new(RefCell::new(node))
    }
}

impl<'i> Node<'i> {
    pub(super) fn get(&self, index: &PathItem<'i>) -> Result<Option<NodeCell<'i>>, String> {
        match (index, self) {
            (PathItem::Index(i), Node::Sequence(s)) => Ok(s.get(*i).map(Rc::clone)),
            (PathItem::MapKey(k), Node::Map(m)) => Ok(m.get(k).map(Rc::clone)),
            (PathItem::FieldName(n), Node::Fields(f)) => Ok(f.get(n).map(Rc::clone)),
            (PathItem::EnumVariant(v1), Node::NewTypeEnumVariant(v2, node)) if v1 == v2 => {
                Ok(Some(Rc::clone(node)))
            }
            _ => Err(format!(
                "path item is expected to be {}, but it was previously defined as {}",
                index.type_name(),
                self.type_name()
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Leaf<'i> {
    InlineSequence(Vec<Value>),
    UnitEnumVariant(&'i str),
    Value(Value),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    PosInt(u64),
    NegInt(i64),
    Float(f64),
    String(String),
}
