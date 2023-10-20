use super::{Path, Value, ValueCell};
use crate::error::{Error, Result};
use indexmap::IndexMap;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::mem;
use std::result::Result as StdResult;

pub trait MergeConflictResolver<E> {
    fn resolve(&self, path: &Path<()>, current: Value, other: Value) -> StdResult<Value, E>;
}

impl<F, E> MergeConflictResolver<E> for F
where
    F: Fn(&Path<()>, Value, Value) -> StdResult<Value, E>,
{
    #[inline]
    fn resolve(&self, path: &Path<()>, current: Value, other: Value) -> StdResult<Value, E> {
        (self)(path, current, other)
    }
}

pub struct PreferCurrentOnConflict;

impl MergeConflictResolver<Infallible> for PreferCurrentOnConflict {
    #[inline]
    fn resolve(
        &self,
        _path: &Path<()>,
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
        &self,
        _path: &Path<()>,
        _current: Value,
        other: Value,
    ) -> StdResult<Value, Infallible> {
        Ok(other)
    }
}

pub struct ErrorOnConflict;

impl MergeConflictResolver<Error> for ErrorOnConflict {
    #[inline]
    fn resolve(&self, path: &Path<()>, _current: Value, _other: Value) -> Result<Value> {
        Err(Error::MergeConflict {
            path: path.to_string(),
        })
    }
}

impl Value {
    #[inline]
    pub fn merge<R, E>(self, conflict_resolver: R, other: impl Into<Value>) -> StdResult<Value, E>
    where
        R: MergeConflictResolver<E>,
    {
        Merge {
            path: Default::default(),
            conflict_resolver,
            _conflict_err_ty: PhantomData,
        }
        .merge_values(self, other.into())
    }
}

impl ValueCell {
    #[inline]
    pub fn merge<R, E>(self, conflict_resolver: R, other: impl Into<Value>) -> StdResult<Value, E>
    where
        R: MergeConflictResolver<E>,
    {
        self.into_value().merge(conflict_resolver, other)
    }
}

struct Merge<R, E> {
    path: Path<'static, ()>,
    conflict_resolver: R,
    _conflict_err_ty: PhantomData<E>,
}

impl<R, E> Merge<R, E>
where
    R: MergeConflictResolver<E>,
{
    fn merge_values(&mut self, current: Value, other: Value) -> StdResult<Value, E> {
        match (current, other) {
            (Value::Sequence(current), Value::Sequence(other)) => {
                self.merge_sequences(current, other)
            }
            (
                Value::Variant(current_name, current_value),
                Value::Variant(other_name, other_value),
            ) if current_name == other_name => {
                self.path.push_variant_name(current_name.to_string());

                let resolved = self
                    .merge_values(current_value.into(), other_value.into())?
                    .into();

                self.path.pop();

                Ok(Value::Variant(current_name, resolved))
            }
            (Value::Map(current), Value::Map(other)) => self
                .merge_maps(current, other, |p, k| p.push_map_key(k))
                .map(Value::Map),
            (Value::Struct(current), Value::Struct(other)) => self
                .merge_maps(current, other, |p, n| p.push_struct_field_name(n))
                .map(Value::Struct),
            (current, other) => self.conflict_resolver.resolve(&self.path, current, other),
        }
    }

    fn merge_sequences(
        &mut self,
        current: Vec<ValueCell>,
        other: Vec<ValueCell>,
    ) -> StdResult<Value, E> {
        let (src, mut dst, is_current_dst) = if current.len() >= other.len() {
            (other, current, true)
        } else {
            (current, other, false)
        };

        for (i, src_value) in src.into_iter().enumerate() {
            if dst[i] != src_value {
                self.path.push_sequence_index(i);

                let dst_value = mem::replace(&mut dst[i], Value::Null.into());

                let (current_value, other_value) = if is_current_dst {
                    (dst_value, src_value)
                } else {
                    (src_value, dst_value)
                };

                let resolved = self
                    .merge_values(current_value.into(), other_value.into())?
                    .into();

                self.path.pop();

                let _ = mem::replace(&mut dst[i], resolved);
            }
        }

        Ok(Value::Sequence(dst))
    }

    fn merge_maps(
        &mut self,
        mut current: IndexMap<String, ValueCell>,
        other: IndexMap<String, ValueCell>,
        push_path: impl Fn(&mut Path<()>, String),
    ) -> StdResult<IndexMap<String, ValueCell>, E> {
        for (other_key, other_value) in other {
            match current.get_mut(&other_key) {
                Some(current_value) if *current_value == other_value => (),
                Some(current_value_ref) => {
                    push_path(&mut self.path, other_key.to_string());

                    let current_value = mem::replace(current_value_ref, Value::Null.into());

                    let resolved = self
                        .merge_values(current_value.into(), other_value.into())?
                        .into();

                    self.path.pop();
                    let _ = mem::replace(current_value_ref, resolved);
                }
                None => {
                    current.insert(other_key, other_value);
                }
            }
        }

        Ok(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    macro_rules! assert_merge {
        ($current:expr, $other:expr, {
            prefer_current: $expected_prefer_current:expr,
            prefer_other: $expected_prefer_other:expr,
            error: $expected_error:expr
        }) => {
            let current = crate::parser::parse(indoc!($current)).unwrap();
            let other = crate::parser::parse(indoc!($other)).unwrap();

            let actual_prefer_current = crate::serializer::serialize(
                &current
                    .clone()
                    .merge(PreferCurrentOnConflict, other.clone())
                    .unwrap()
                    .into_cell(),
                Default::default(),
            )
            .unwrap();

            let actual_prefer_other = crate::serializer::serialize(
                &current
                    .clone()
                    .merge(PreferOtherOnConflict, other.clone())
                    .unwrap()
                    .into_cell(),
                Default::default(),
            )
            .unwrap();

            let actual_error = current.clone().merge(ErrorOnConflict, other.clone()).err();

            assert_eq!(
                actual_prefer_current,
                indoc!($expected_prefer_current),
                "prefer current"
            );
            assert_eq!(
                actual_prefer_other,
                indoc!($expected_prefer_other),
                "prefer other"
            );
            assert_eq!(actual_error, $expected_error, "error path");
        };
    }

    #[test]
    fn top_level_conflict() {
        assert_merge! {
            "> = 42",
            "> = 43",
            {
                prefer_current: "> = 42",
                prefer_other: "> = 43",
                error: Some(Error::MergeConflict { path: ">".into() })
            }
        }

        assert_merge! {
            "> = [1, 2, 3]",
            "> = 42",
            {
                prefer_current: "> = [1, 2, 3]",
                prefer_other: "> = 42",
                error: Some(Error::MergeConflict { path: ">".into() })
            }
        }

        assert_merge! {
            "> foo > bar > baz = 42",
            "> `foo` = 42",
            {
                prefer_current: "> foo > bar > baz = 42",
                prefer_other: "> `foo` = 42",
                error: Some(Error::MergeConflict { path: ">".into() })
            }
        }
    }

    #[test]
    fn sequence_identical() {
        assert_merge! {
            "
                > [0] > foo > bar = 123

                > [1] = 42

                > [2] > `baz` = 43\
            ",
            "
                > [0] > foo > bar = 123

                > [1] = 42

                > [2] > `baz` = 43\
            ",
            {
                prefer_current:
                "
                    > [0] > foo > bar = 123

                    > [1] = 42

                    > [2] > `baz` = 43\
                ",

                prefer_other:
                "
                    > [0] > foo > bar = 123

                    > [1] = 42

                    > [2] > `baz` = 43\
                ",

                error: None
            }
        }
    }

    #[test]
    fn sequence_empty() {
        assert_merge! {
            "> = []",
            "> = []",
            {
                prefer_current: "> = []",
                prefer_other: "> = []",
                error: None
            }
        }
    }

    #[test]
    fn sequence_current_longer() {
        assert_merge! {
            "> = [1, 2, 3]",
            "> = []",
            {
                prefer_current: "> = [1, 2, 3]",
                prefer_other: "> = [1, 2, 3]",
                error: None
            }
        }

        assert_merge! {
            "> = [1, 2, 3]",
            "> = [1, 2]",
            {
                prefer_current: "> = [1, 2, 3]",
                prefer_other: "> = [1, 2, 3]",
                error: None
            }
        }
    }

    #[test]
    fn sequence_other_longer() {
        assert_merge! {
            "> = []",
            "> = [1, 2, 3]",
            {
                prefer_current: "> = [1, 2, 3]",
                prefer_other: "> = [1, 2, 3]",
                error: None
            }
        }

        assert_merge! {
            "> = [1, 2]",
            "> = [1, 2, 3]",
            {
                prefer_current: "> = [1, 2, 3]",
                prefer_other: "> = [1, 2, 3]",
                error: None
            }
        }
    }

    #[test]
    fn sequence_conflict() {
        assert_merge! {
            "> = [1, 4, 3]",
            "> = [1, 2, 3]",
            {
                prefer_current: "> = [1, 4, 3]",
                prefer_other: "> = [1, 2, 3]",
                error: Some(Error::MergeConflict { path: "> [1]".into() })
            }
        }

        assert_merge! {
            "
                > [0] = [123]

                > [1] = 42

                > [2] > `baz` = 43\
            ",
            "
                > [0] = [`qux`]

                > [1] = 43

                > [2] > `baz` > foo = 43\
            ",
            {
                prefer_current:
                "
                    > [0] = [123]

                    > [1] = 42

                    > [2] > `baz` = 43\
                ",

                prefer_other:
                "
                    > [0] = [`qux`]

                    > [1] = 43

                    > [2] > `baz` > foo = 43\
                ",

                error: Some(Error::MergeConflict { path: "> [0] > [0]".into() })
            }
        }
    }

    #[test]
    fn variant_different_name() {
        assert_merge! {
            "> `foo` = 42",
            "> `bar` = 42",
            {
                prefer_current: "> `foo` = 42",
                prefer_other: "> `bar` = 42",
                error: Some(Error::MergeConflict { path: ">".into() })
            }
        }
    }

    #[test]
    fn variant_different_value() {
        assert_merge! {
            "> `foo` = 42",
            "> `foo` = 43",
            {
                prefer_current: "> `foo` = 42",
                prefer_other: "> `foo` = 43",
                error: Some(Error::MergeConflict { path: "> `foo`".into() })
            }
        }
    }

    #[test]
    fn map_current_has_more_keys() {
        assert_merge! {
            "
                > [\"foo\"] = 42

                > [\"bar\"] = 43

                > [\"baz\"] = 44\
            ",
            "
                > [\"foo\"] = 42

                > [\"baz\"] = 44\
            ",
            {
                prefer_current:
                "
                    > [\"foo\"] = 42

                    > [\"bar\"] = 43

                    > [\"baz\"] = 44\
                ",
                prefer_other:
                "
                    > [\"foo\"] = 42

                    > [\"bar\"] = 43

                    > [\"baz\"] = 44\
                ",

                error: None
            }
        }
    }

    #[test]
    fn map_other_has_more_keys() {
        assert_merge! {
            "
                > [\"foo\"] = 42

                > [\"baz\"] = 44\
            ",
            "
                > [\"foo\"] = 42

                > [\"bar\"] = 43

                > [\"baz\"] = 44\
            ",
            {
                prefer_current:
                "
                    > [\"foo\"] = 42

                    > [\"baz\"] = 44

                    > [\"bar\"] = 43\
                ",
                prefer_other:
                "
                    > [\"foo\"] = 42

                    > [\"baz\"] = 44

                    > [\"bar\"] = 43\
                ",

                error: None
            }
        }
    }

    #[test]
    fn map_identical() {
        assert_merge! {
            "
                > [\"foo\"] = 42

                > [\"baz\"] = 44\
            ",
            "
                > [\"foo\"] = 42

                > [\"baz\"] = 44\
            ",
            {
                prefer_current:
                "
                    > [\"foo\"] = 42

                    > [\"baz\"] = 44\
                ",
                prefer_other:
                "
                    > [\"foo\"] = 42

                    > [\"baz\"] = 44\
                ",

                error: None
            }
        }
    }

    #[test]
    fn map_conflict() {
        assert_merge! {
            "
                > [\"foo\"] > [\"bar\"] = 42

                > [\"baz\"] = `foo`\
            ",
            "
                > [\"foo\"] > [\"qux\"] = 43

                > [\"baz\"] = \"hello\"

                > [\"quz\"] = 0\
            ",
            {
                prefer_current:
                "
                    > [\"foo\"] > [\"bar\"] = 42

                    > [\"foo\"] > [\"qux\"] = 43

                    > [\"baz\"] = `foo`

                    > [\"quz\"] = 0\
                ",
                prefer_other:
                "
                    > [\"foo\"] > [\"bar\"] = 42

                    > [\"foo\"] > [\"qux\"] = 43

                    > [\"baz\"] = \"hello\"

                    > [\"quz\"] = 0\
                ",

                error: Some(Error::MergeConflict { path: "> [\"baz\"]".into() })
            }
        }
    }

    #[test]
    fn struct_current_has_more_fields() {
        assert_merge! {
            "
                > foo = 42

                > bar = 43

                > baz = 44\
            ",
            "
                > foo = 42

                > baz = 44\
            ",
            {
                prefer_current:
                "
                    > foo = 42

                    > bar = 43

                    > baz = 44\
                ",
                prefer_other:
                "
                    > foo = 42

                    > bar = 43

                    > baz = 44\
                ",

                error: None
            }
        }
    }

    #[test]
    fn struct_other_has_more_fields() {
        assert_merge! {
            "
                > foo = 42

                > baz = 44\
            ",
            "
                > foo = 42

                > bar = 43

                > baz = 44\
            ",
            {
                prefer_current:
                "
                    > foo = 42

                    > baz = 44

                    > bar = 43\
                ",
                prefer_other:
                "
                    > foo = 42

                    > baz = 44

                    > bar = 43\
                ",

                error: None
            }
        }
    }

    #[test]
    fn struct_identical() {
        assert_merge! {
            "
                > foo = 42

                > baz = 44\
            ",
            "
                > foo = 42

                > baz = 44\
            ",
            {
                prefer_current:
                "
                    > foo = 42

                    > baz = 44\
                ",
                prefer_other:
                "
                    > foo = 42

                    > baz = 44\
                ",

                error: None
            }
        }
    }

    #[test]
    fn struct_conflict() {
        assert_merge! {
            "
                > foo > bar = 42

                > baz = `foo`\
            ",
            "
                > foo > qux = 43

                > baz = \"hello\"

                > quz = 0\
            ",
            {
                prefer_current:
                "
                    > foo > bar = 42

                    > foo > qux = 43

                    > baz = `foo`

                    > quz = 0\
                ",
                prefer_other:
                "
                    > foo > bar = 42

                    > foo > qux = 43

                    > baz = \"hello\"

                    > quz = 0\
                ",

                error: Some(Error::MergeConflict { path: "> baz".into() })
            }
        }
    }
}
