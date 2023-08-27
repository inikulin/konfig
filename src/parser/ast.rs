use super::type_name::TypeName;
use super::PathItem;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Sequence(Vec<NodeCell>),
    Map(HashMap<String, NodeCell>),
    NewTypeEnumVariant(String, NodeCell),
    Fields(HashMap<String, NodeCell>),
    Leaf(Leaf),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Leaf {
    InlineSequence(Vec<Value>),
    UnitEnumVariant(String),
    Value(Value),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    PosInt(u64),
    NegInt(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct NodeCell(Rc<RefCell<Node>>);

impl Node {
    pub(super) fn is_multitenant(&self) -> bool {
        matches!(self, Node::Sequence(_) | Node::Map(_) | Node::Fields(_))
    }
}

impl Node {
    pub(super) fn get(&self, index: &PathItem) -> Result<Option<NodeCell>, String> {
        match (index, self) {
            (PathItem::Index(idx), Node::Sequence(seq)) => {
                Ok(seq.get(*idx).map(NodeCell::rc_clone))
            }
            (PathItem::MapKey(key), Node::Map(map)) => Ok(map.get(key).map(NodeCell::rc_clone)),
            (PathItem::FieldName(name), Node::Fields(fields)) => {
                Ok(fields.get(*name).map(NodeCell::rc_clone))
            }
            (PathItem::EnumVariant(var1), Node::NewTypeEnumVariant(var2, node)) if var1 == var2 => {
                Ok(Some(node.rc_clone()))
            }
            _ => Err(format!(
                "path item is expected to be {}, but it was previously defined as {}",
                index.type_name(),
                self.type_name()
            )),
        }
    }
}

impl From<NodeCell> for Node {
    fn from(cell: NodeCell) -> Self {
        #[cfg(test)]
        node_cell_safety_checks::assert_not_parsing();

        // SAFETY: it's guaranteed that `NodeCell` has exclusive ownership of the `Node` when
        // parsing is complete.
        let ref_cell = unsafe { ptr::read(Rc::as_ptr(&cell.0)) };

        mem::forget(cell.0);

        ref_cell.into_inner()
    }
}

impl NodeCell {
    #[inline]
    pub(super) fn rc_clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }

    #[inline]
    pub(super) fn borrow(&self) -> Ref<Node> {
        self.0.borrow()
    }

    #[inline]
    pub(super) fn borrow_mut(&self) -> RefMut<Node> {
        self.0.borrow_mut()
    }
}

impl Deref for NodeCell {
    type Target = Node;

    #[inline]
    fn deref(&self) -> &Self::Target {
        #[cfg(test)]
        node_cell_safety_checks::assert_not_parsing();

        // SAFETY: it's guaranteed that `NodeCell` has exclusive ownership of the `Node` when
        // parsing is complete.
        unsafe { &*self.0.as_ptr() }
    }
}

impl DerefMut for NodeCell {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[cfg(test)]
        node_cell_safety_checks::assert_not_parsing();

        // SAFETY: it's guaranteed that `NodeCell` has exclusive ownership of the `Node` when
        // parsing is complete.
        unsafe { &mut *self.0.as_ptr() }
    }
}

impl Clone for NodeCell {
    #[inline]
    fn clone(&self) -> Self {
        self.deref().clone().into()
    }
}

impl From<Node> for NodeCell {
    fn from(node: Node) -> Self {
        Self(Rc::new(RefCell::new(node)))
    }
}

// SAFETY: having `Rc<RefCell<Node>>` allows us to construct AST in the safe manner. It is guaranteed
// that parsing is done in a single thread and AST is not exposed to the external code until parsing
// is complete. After releasing the AST to the external code all the `Rc` and `RefCell` API is hidden,
// so exclusive ownership is guraranteed allowing us to ignore those containers. Compile time test
// ensures that `Node` is `Send` and `Sync` itself.
unsafe impl Send for NodeCell {}
unsafe impl Sync for NodeCell {}

#[cfg(test)]
pub(super) mod node_cell_safety_checks {
    use super::*;
    use std::cell::Cell;
    use std::thread_local;

    thread_local! {
        pub(crate) static IS_PARSING: Cell<bool> = Default::default();
    }

    pub(super) fn assert_not_parsing() {
        IS_PARSING.with(|is_parsing| {
            assert!(
                !is_parsing.get(),
                "parser should not use `NodeCell` API that assumes exclusive ownership"
            );
        });
    }

    fn _assert_node_send_sync() {
        fn assert<S: Send + Sync>() {}

        assert::<Node>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_from_cell() {
        let node = Node::Leaf(Leaf::InlineSequence(vec![
            Value::PosInt(42),
            Value::PosInt(43),
            Value::PosInt(44),
        ]));

        let cell = NodeCell::from(node.clone());
        let actual = Node::from(cell);

        assert_eq!(actual, node);
    }
}
