use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Sequence(Vec<ValueCell>),
    Map(HashMap<String, ValueCell>),
    Struct(HashMap<String, ValueCell>),
    Variant(String, ValueCell),
    SequenceOfPrimitives(Vec<Primitive>),
    Primitive(Primitive),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Null,
    Bool(bool),
    PosInt(u64),
    NegInt(i64),
    Float(f64),
    String(String),
    UnitVariant(String),
}

#[derive(Debug, PartialEq)]
pub struct ValueCell(Rc<RefCell<Value>>);

impl Value {
    pub(crate) fn is_multitenant(&self) -> bool {
        matches!(self, Value::Sequence(_) | Value::Map(_) | Value::Struct(_))
    }
}

impl From<ValueCell> for Value {
    fn from(cell: ValueCell) -> Self {
        #[cfg(test)]
        value_cell_safety_checks::assert_not_parsing();

        // SAFETY: it's guaranteed that `ValueCell` has exclusive ownership of the `Value` when
        // parsing is complete.
        let ref_cell = unsafe { ptr::read(Rc::as_ptr(&cell.0)) };

        mem::forget(cell.0);

        ref_cell.into_inner()
    }
}

impl ValueCell {
    #[inline]
    pub(super) fn rc_clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }

    #[inline]
    pub(super) fn borrow(&self) -> Ref<Value> {
        self.0.borrow()
    }

    #[inline]
    pub(super) fn borrow_mut(&self) -> RefMut<Value> {
        self.0.borrow_mut()
    }
}

impl Deref for ValueCell {
    type Target = Value;

    #[inline]
    fn deref(&self) -> &Self::Target {
        #[cfg(test)]
        value_cell_safety_checks::assert_not_parsing();

        // SAFETY: it's guaranteed that `ValueCell` has exclusive ownership of the `Value` when
        // parsing is complete.
        unsafe { &*self.0.as_ptr() }
    }
}

impl DerefMut for ValueCell {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[cfg(test)]
        value_cell_safety_checks::assert_not_parsing();

        // SAFETY: it's guaranteed that `ValueCell` has exclusive ownership of the `Value` when
        // parsing is complete.
        unsafe { &mut *self.0.as_ptr() }
    }
}

impl Clone for ValueCell {
    #[inline]
    fn clone(&self) -> Self {
        self.deref().clone().into()
    }
}

impl From<Value> for ValueCell {
    fn from(value: Value) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
}

// SAFETY: having `Rc<RefCell<Value>>` allows us to construct AST in the safe manner. It is guaranteed
// that parsing is done in a single thread and AST is not exposed to the external code until parsing
// is complete. After releasing the AST to the external code all the `Rc` and `RefCell` API is hidden,
// so exclusive ownership is guraranteed allowing us to ignore those containers. Compile time test
// ensures that `Value` is `Send` and `Sync` itself.
unsafe impl Send for ValueCell {}
unsafe impl Sync for ValueCell {}

#[cfg(test)]
pub(super) mod value_cell_safety_checks {
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
                "parser should not use `ValueCell` API that assumes exclusive ownership"
            );
        });
    }

    fn _assert_value_send_sync() {
        fn assert<S: Send + Sync>() {}

        assert::<Value>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_from_cell() {
        let value = Value::SequenceOfPrimitives(vec![
            Primitive::PosInt(42),
            Primitive::PosInt(43),
            Primitive::PosInt(44),
        ]);

        let cell = ValueCell::from(value.clone());
        let actual = Value::from(cell);

        assert_eq!(actual, value);
    }
}
