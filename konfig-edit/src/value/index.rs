use super::{Value, ValueCell};
use std::ops::{Index, IndexMut};

static ERR_INDEX_OUT_OF_BOUNDS: &str = "index is out of bounds";
static ERR_CANT_INDEX_BY_USIZE: &str = "value is not a sequence and can't be indexed by `usize`";
static ERR_CANT_INDEX_BY_STR: &str =
    "value is not a map, a structure, or a variant and can't be indexed by `&str`";
static ERR_NO_ENTRY_FOR_KEY: &str = "no entry found for key";

impl Index<usize> for Value {
    type Output = ValueCell;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Value::Sequence(s) => s.get(index).expect(ERR_INDEX_OUT_OF_BOUNDS),
            _ => panic!("{}", ERR_CANT_INDEX_BY_USIZE),
        }
    }
}

impl IndexMut<usize> for Value {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Value::Sequence(s) => s.get_mut(index).expect(ERR_INDEX_OUT_OF_BOUNDS),
            _ => panic!("{}", ERR_CANT_INDEX_BY_USIZE),
        }
    }
}

impl Index<&str> for Value {
    type Output = ValueCell;

    fn index(&self, index: &str) -> &Self::Output {
        match self {
            Value::Struct(m) | Value::Map(m) => m.get(index).expect(ERR_NO_ENTRY_FOR_KEY),
            Value::Variant(name, value) => {
                if index_matches_variant(index, name) {
                    value
                } else {
                    panic!("{}", ERR_NO_ENTRY_FOR_KEY)
                }
            }
            _ => panic!("{}", ERR_CANT_INDEX_BY_STR),
        }
    }
}

impl IndexMut<&str> for Value {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match self {
            Value::Struct(m) | Value::Map(m) => m.get_mut(index).expect(ERR_NO_ENTRY_FOR_KEY),
            Value::Variant(name, ref mut value) => {
                if index_matches_variant(index, name) {
                    value
                } else {
                    panic!("{}", ERR_NO_ENTRY_FOR_KEY)
                }
            }
            _ => panic!("{}", ERR_CANT_INDEX_BY_STR),
        }
    }
}

impl Index<String> for Value {
    type Output = ValueCell;

    #[inline]
    fn index(&self, index: String) -> &Self::Output {
        Index::index(self, index.as_str())
    }
}

impl IndexMut<String> for Value {
    #[inline]
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        IndexMut::index_mut(self, index.as_str())
    }
}

fn index_matches_variant(index: &str, variant_name: &str) -> bool {
    index
        .strip_prefix('`')
        .and_then(|index| index.strip_suffix('`'))
        .map(|index| index == variant_name)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn index() {
        let value = parse("> [0] > foo > ['bar'] > `baz` = 42").unwrap();

        assert_eq!(value[0]["foo"]["bar".to_string()]["`baz`"], Value::UInt(42));
    }

    #[test]
    fn index_mut() {
        let mut value = parse("> [0] > foo > ['bar'] > `baz` = 42").unwrap();

        value[0]["foo"]["bar"]["`baz`"] = Value::UInt(43).into();

        assert_eq!(value[0]["foo"]["bar".to_string()]["`baz`"], Value::UInt(43));
    }

    #[test]
    #[should_panic(expected = "index is out of bounds")]
    fn index_out_of_bounds() {
        let value = parse("> [0] = 42").unwrap();
        let _ = value[1];
    }

    #[test]
    #[should_panic(expected = "index is out of bounds")]
    fn index_mut_out_of_bounds() {
        let mut value = parse("> [0] = 42").unwrap();

        value[1] = Value::Null.into();
    }

    #[test]
    #[should_panic(expected = "value is not a sequence and can't be indexed by `usize`")]
    fn cant_index_by_usize() {
        let value = parse("> ['foo'] = 42").unwrap();
        let _ = value[0];
    }

    #[test]
    #[should_panic(expected = "value is not a sequence and can't be indexed by `usize`")]
    fn cant_index_mut_by_usize() {
        let mut value = parse("> ['foo'] = 42").unwrap();

        value[0] = Value::Null.into();
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn map_index_no_entry_for_key() {
        let value = parse("> ['foo'] = 42").unwrap();
        let _ = value["bar"];
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn map_index_mut_no_entry_for_key() {
        let mut value = parse("> ['foo'] = 42").unwrap();

        value["bar"] = Value::Null.into();
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn variant_index_no_entry_for_key() {
        let value = parse("> `foo` = 42").unwrap();
        let _ = value["`bar`"];
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn variant_index_no_backticks_no_entry_for_key() {
        let value = parse("> `foo` = 42").unwrap();
        let _ = value["foo"];
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn variant_index_mut_no_entry_for_key() {
        let mut value = parse("> `foo` = 42").unwrap();

        value["`bar`"] = Value::Null.into();
    }

    #[test]
    #[should_panic(
        expected = "value is not a map, a structure, or a variant and can't be indexed by `&str`"
    )]
    fn cant_index_by_str() {
        let value = parse("> [0] = 42").unwrap();
        let _ = value["foo"];
    }

    #[test]
    #[should_panic(
        expected = "value is not a map, a structure, or a variant and can't be indexed by `&str`"
    )]
    fn cant_index_mut_by_str() {
        let mut value = parse("> [0] = 42").unwrap();

        value["foo"] = Value::Null.into();
    }
}
