mod conv;

#[cfg(feature = "serde")]
mod to_value;

#[cfg(feature = "serde")]
mod from_value;

#[cfg(feature = "serde")]
mod serde;

use crate::parser::LexicalInfo;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::rc::Rc;

#[cfg(feature = "serde")]
pub use self::to_value::{to_value, Serializer};

#[cfg(feature = "serde")]
pub use self::from_value::from_value;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Value {
    #[default]
    Null,
    Bool(bool),
    UInt(u64),
    Int(i64),
    Float(f64),
    String(String),
    UnitVariant(String),
    Sequence(Vec<ValueCell>),
    Map(HashMap<String, ValueCell>),
    Struct(HashMap<String, ValueCell>),
    Variant(String, ValueCell),
}

impl From<ValueCell> for Value {
    fn from(cell: ValueCell) -> Self {
        #[cfg(debug_assertions)]
        value_cell_safety_checks::assert_not_parsing();

        // SAFETY: it's guaranteed that `ValueCell` has exclusive ownership of the `Value` when
        // parsing is complete.
        let ref_cell = unsafe { ptr::read(Rc::as_ptr(&cell.0)) };

        mem::forget(cell.0);

        ref_cell.into_inner().value
    }
}

#[derive(Debug)]
pub(super) struct ValueCellInternal {
    pub(super) value: Value,
    pub(super) lexical_info: LexicalInfo,
}

pub struct ValueCell(Rc<RefCell<ValueCellInternal>>);

impl ValueCell {
    pub(super) fn new(value: Value, lexical_info: LexicalInfo) -> Self {
        Self(Rc::new(RefCell::new(ValueCellInternal {
            value,
            lexical_info,
        })))
    }

    #[inline]
    pub(super) fn rc_clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }

    #[inline]
    pub(super) fn borrow(&self) -> Ref<ValueCellInternal> {
        self.0.borrow()
    }

    #[inline]
    pub(super) fn borrow_mut(&self) -> RefMut<ValueCellInternal> {
        self.0.borrow_mut()
    }

    #[inline]
    pub fn into_value(self) -> Value {
        self.into()
    }

    #[inline]
    pub fn lexical_info(&self) -> &LexicalInfo {
        &self.internal().lexical_info
    }

    #[inline]
    pub fn lexical_info_mut(&mut self) -> &LexicalInfo {
        &self.internal_mut().lexical_info
    }

    #[inline]
    fn internal(&self) -> &ValueCellInternal {
        #[cfg(debug_assertions)]
        value_cell_safety_checks::assert_not_parsing();

        // SAFETY: it's guaranteed that `ValueCell` has exclusive ownership of the
        // `ValueCellInternal` when parsing is complete.
        unsafe { self.0.as_ptr().as_ref().unwrap() }
    }

    #[inline]
    fn internal_mut(&mut self) -> &mut ValueCellInternal {
        #[cfg(debug_assertions)]
        value_cell_safety_checks::assert_not_parsing();

        // SAFETY: it's guaranteed that `ValueCell` has exclusive ownership of the
        // `ValueCellInternal` when parsing is complete.
        unsafe { self.0.as_ptr().as_mut().unwrap() }
    }
}

impl Deref for ValueCell {
    type Target = Value;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.internal().value
    }
}

impl fmt::Debug for ValueCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl DerefMut for ValueCell {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.internal_mut().value
    }
}

impl PartialEq<ValueCell> for ValueCell {
    fn eq(&self, other: &ValueCell) -> bool {
        **self == **other
    }
}

impl PartialEq<Value> for ValueCell {
    fn eq(&self, other: &Value) -> bool {
        **self == *other
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
        // NOTE: it is technically safe to use this API in parsing, but we still want to forbid
        // it to ensure that lexical information is explicitly assigned by the parser for each
        // node.
        #[cfg(debug_assertions)]
        value_cell_safety_checks::assert_not_parsing();

        Self::new(value, Default::default())
    }
}

// SAFETY: having `Rc<RefCell<Value>>` allows us to construct AST in the safe manner. It is guaranteed
// that parsing is done in a single thread and AST is not exposed to the external code until parsing
// is complete. After releasing the AST to the external code all the `Rc` and `RefCell` API is hidden,
// so exclusive ownership is guraranteed allowing us to ignore those containers. Compile time test
// ensures that `Value` is `Send` and `Sync` itself.
unsafe impl Send for ValueCell {}
unsafe impl Sync for ValueCell {}

#[cfg(debug_assertions)]
pub(super) mod value_cell_safety_checks {
    use super::*;
    use std::cell::Cell;
    use std::thread_local;

    thread_local! {
        static IS_PARSING: Cell<bool> = Default::default();
    }

    pub(crate) struct ParsingGuard;

    impl ParsingGuard {
        pub(crate) fn new() -> Self {
            IS_PARSING.with(|is_parsing| is_parsing.set(true));

            Self
        }
    }

    impl Drop for ParsingGuard {
        fn drop(&mut self) {
            IS_PARSING.with(|is_parsing| is_parsing.set(false));
        }
    }

    pub(super) fn assert_not_parsing() {
        IS_PARSING.with(|is_parsing| {
            assert!(!is_parsing.get(), "parser should not use this API");
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
        let value = Value::Sequence(vec![
            Value::UInt(42).into(),
            Value::UInt(43).into(),
            Value::UInt(44).into(),
        ]);

        let cell = ValueCell::from(value.clone());
        let actual = Value::from(cell);

        assert_eq!(actual, value);
    }
}
