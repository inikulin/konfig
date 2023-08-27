use super::type_name::TypeName;
use super::PathItem;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type NodeCell = Rc<RefCell<Node>>;

#[derive(Debug, PartialEq)]
pub enum Node {
    Sequence(Vec<NodeCell>),
    Map(HashMap<String, NodeCell>),
    NewTypeEnumVariant(String, NodeCell),
    Fields(HashMap<String, NodeCell>),
    Leaf(Leaf),
}

impl Node {
    pub(super) fn is_multitenant(&self) -> bool {
        matches!(self, Node::Sequence(_) | Node::Map(_) | Node::Fields(_))
    }
}

impl From<Node> for NodeCell {
    fn from(node: Node) -> Self {
        Rc::new(RefCell::new(node))
    }
}

impl Node {
    pub(super) fn get(&self, index: &PathItem) -> Result<Option<NodeCell>, String> {
        match (index, self) {
            (PathItem::Index(idx), Node::Sequence(seq)) => Ok(seq.get(*idx).map(Rc::clone)),
            (PathItem::MapKey(key), Node::Map(map)) => Ok(map.get(key).map(Rc::clone)),
            (PathItem::FieldName(name), Node::Fields(fields)) => {
                Ok(fields.get(*name).map(Rc::clone))
            }
            (PathItem::EnumVariant(var1), Node::NewTypeEnumVariant(var2, node)) if var1 == var2 => {
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
pub enum Leaf {
    InlineSequence(Vec<Value>),
    UnitEnumVariant(String),
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
