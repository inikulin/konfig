use super::{Path, Value, ValueCell};
use crate::error::{Error, Result};
use std::convert::Infallible;
use std::result::Result as StdResult;

pub trait MergeConflictResolver<E> {
    fn resolve(&mut self, path: &Path, current: Value, other: Value) -> StdResult<Value, E>;
}

impl<F, E> MergeConflictResolver<E> for F
where
    F: FnMut(&Path, Value, Value) -> StdResult<Value, E>,
{
    #[inline]
    fn resolve(&mut self, path: &Path, current: Value, other: Value) -> StdResult<Value, E> {
        (self)(path, current, other)
    }
}

pub struct PreferCurrentOnConflict;

impl MergeConflictResolver<Infallible> for PreferCurrentOnConflict {
    #[inline]
    fn resolve(
        &mut self,
        _path: &Path,
        current: Value,
        _other: Value,
    ) -> StdResult<Value, Infallible> {
        Ok(current)
    }
}

pub struct PreferOtherOnConflict;

impl MergeConflictResolver<Infallible> for PreferOtherOnConflict {
    #[inline]
    fn resolve(
        &mut self,
        _path: &Path,
        _current: Value,
        other: Value,
    ) -> StdResult<Value, Infallible> {
        Ok(other)
    }
}

pub struct ErrorOnConflict;

impl MergeConflictResolver<Error> for ErrorOnConflict {
    #[inline]
    fn resolve(&mut self, path: &Path, _current: Value, _other: Value) -> Result<Value> {
        Err(Error::MergeConflict {
            path: path.to_string(),
        })
    }
}

impl Value {
    #[inline]
    pub fn merge<R, E>(
        self,
        mut merge_conflict_resolver: R,
        other: impl Into<Value>,
    ) -> StdResult<Value, E>
    where
        R: MergeConflictResolver<E>,
    {
        merge(
            &mut Default::default(),
            self,
            other.into(),
            &mut merge_conflict_resolver,
        )
    }
}

impl ValueCell {
    #[inline]
    pub fn merge<R, E>(
        self,
        merge_conflict_resolver: R,
        other: impl Into<Value>,
    ) -> StdResult<Value, E>
    where
        R: MergeConflictResolver<E>,
    {
        self.into_value().merge(merge_conflict_resolver, other)
    }
}

fn merge<R, E>(
    path: &mut Path,
    current: Value,
    other: Value,
    merge_conflict_resolver: &mut R,
) -> StdResult<Value, E>
where
    R: MergeConflictResolver<E>,
{
    match (current, other) {
        (Value::Sequence(c), Value::Sequence(o)) => {
            merge_sequence(path, c, o, merge_conflict_resolver)
        }
        (current, other) => merge_conflict_resolver.resolve(path, current, other),
    }
}

fn merge_sequence<R, E>(
    path: &mut Path,
    current: Vec<ValueCell>,
    other: Vec<ValueCell>,
    merge_conflict_resolver: &mut R,
) -> StdResult<Value, E>
where
    R: MergeConflictResolver<E>,
{
    let mut merged = Vec::with_capacity(current.len().max(other.len()));
    let mut current = current.into_iter();
    let mut other = other.into_iter();

    loop {
        match (current.next(), other.next()) {
            (Some(c), Some(o)) if c == o => merged.push(c),
            (Some(c), Some(o)) => {
                path.push_sequence_index(merged.len());

                let resolved = merge(path, c.into(), o.into(), merge_conflict_resolver)?.into();

                merged.push(resolved);
                path.pop();
            }
            (None, Some(v)) | (Some(v), None) => merged.push(v),
            (None, None) => break,
        }
    }

    Ok(Value::Sequence(merged))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    macro_rules! assert_merge {
        ($current:expr, $other:expr, {
            prefer_current: $expected_prefer_current:expr,
            prefer_other: $expected_prefer_other:expr,
            error_path: $expected_error_path:expr
        }) => {
            let current = crate::parser::parse(indoc!($current)).unwrap();
            let other = crate::parser::parse(indoc!($other)).unwrap();

            let actual_prefer_current = current
                .clone()
                .merge(PreferCurrentOnConflict, other.clone())
                .unwrap()
                .into_cell()
                .to_konfig()
                .unwrap();

            let actual_prefer_other = current
                .clone()
                .merge(PreferOtherOnConflict, other.clone())
                .unwrap()
                .into_cell()
                .to_konfig()
                .unwrap();

            let actual_error = current
                .clone()
                .merge(ErrorOnConflict, other.clone())
                .unwrap_err();

            assert_eq!(actual_prefer_current, indoc!($expected_prefer_current));
            assert_eq!(actual_prefer_other, indoc!($expected_prefer_other));

            assert_eq!(
                actual_error,
                Error::MergeConflict {
                    path: $expected_error_path.to_string()
                }
            );
        };
    }

    #[test]
    fn primitives() {
        assert_merge! {
            "> = 42",
            "> = 43",
            {
                prefer_current: "> = 42",
                prefer_other: "> = 43",
                error_path: ">"
            }
        }
    }
}
