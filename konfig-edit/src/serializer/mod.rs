pub mod components;
pub mod formatting;

use self::components::{escape_docs, write_escaped_str, write_float, write_int};
use self::formatting::FormattingOptions;
use crate::error::{Error, Result};
use crate::value::{Path, Value, ValueCell};
use indexmap::IndexMap;

pub fn serialize(value: &ValueCell, formatting: FormattingOptions) -> Result<String> {
    let mut serializer = KonfigSerializer {
        out: Default::default(),
        path: Default::default(),
        have_docs_after: false,
        formatting,
    };

    serializer.serialize(value)?;

    // NOTE: trim last expression separator. It's much simpler to implement it this way,
    // even though it's not the most elegant approach.s
    if !serializer.have_docs_after {
        let removed = (serializer.out.pop(), serializer.out.pop());

        debug_assert_eq!(removed, (Some('\n'), Some('\n')));
    }

    Ok(serializer.out)
}

struct KonfigSerializer<'v> {
    out: String,
    path: Path<'v>,
    have_docs_after: bool,
    formatting: FormattingOptions,
}

impl<'v> KonfigSerializer<'v> {
    fn serialize(&mut self, value: &'v ValueCell) -> Result<()> {
        self.have_docs_after = false;

        let docs_before = &value.lexical_info().docs_before;

        self.out.push_str(&escape_docs(
            docs_before,
            self.formatting.doc_line_escape.as_ref(),
        ));

        match **value {
            Value::Null => self.write_rhs_infallible(|s| s.write_null()),
            Value::Bool(v) => self.write_rhs_infallible(|s| s.write_bool(v)),
            Value::Int(v) => self.write_rhs(|s| write_int(&mut s.out, v).map_err(Error::custom)),
            Value::UInt(v) => self.write_rhs(|s| write_int(&mut s.out, v).map_err(Error::custom)),
            Value::Float(v) => self.write_rhs(|s| write_float(&mut s.out, v)),
            Value::String(ref v) => self.write_rhs(|s| s.write_string(v)),
            Value::UnitVariant(ref v) => self.write_rhs(|s| s.write_unit_variant(v)),
            Value::Sequence(ref v) if is_all_primitive(v) => {
                self.serialize_sequence_of_primitives(v)
            }
            Value::Sequence(ref v) => self.serialize_sequence(v),
            Value::Map(ref v) => self.serialize_map(v),
            Value::Struct(ref v) => self.serialize_struct(v),
            Value::Variant(ref n, ref v) => self.serialize_variant(n, v),
        }?;

        let docs_after = &value.lexical_info().docs_after;

        if !docs_after.is_empty() {
            self.out.push_str(&escape_docs(
                docs_after,
                self.formatting.doc_line_escape.as_ref(),
            ));

            self.have_docs_after = true;
        }

        Ok(())
    }

    #[inline]
    fn write_null(&mut self) {
        self.out.push_str("null")
    }

    #[inline]
    fn write_bool(&mut self, v: bool) {
        self.out.push_str(if v { "true" } else { "false" })
    }

    fn write_string(&mut self, v: &str) -> Result<()> {
        self.out.push('"');
        write_escaped_str(&mut self.out, v).map_err(Error::custom)?;
        self.out.push('"');

        Ok(())
    }

    fn write_unit_variant(&mut self, v: &str) -> Result<()> {
        validate_ident(v)?;
        self.out.push('`');
        self.out.push_str(v);
        self.out.push('`');

        Ok(())
    }

    fn serialize_sequence(&mut self, seq: &'v [ValueCell]) -> Result<()> {
        for (idx, v) in seq.iter().enumerate() {
            self.path.push_sequence_index(idx);
            self.serialize(v)?;
            self.path.pop();
        }

        Ok(())
    }

    fn serialize_sequence_of_primitives(&mut self, seq: &'v [ValueCell]) -> Result<()> {
        let last = seq.len().saturating_sub(1);

        self.path.write(&mut self.out).map_err(Error::custom)?;
        self.out.push_str(" = [");

        for (idx, v) in seq.iter().enumerate() {
            match **v {
                Value::Null => self.write_null(),
                Value::Bool(v) => self.write_bool(v),
                Value::Int(v) => write_int(&mut self.out, v).map_err(Error::custom)?,
                Value::UInt(v) => write_int(&mut self.out, v).map_err(Error::custom)?,
                Value::Float(v) => write_float(&mut self.out, v)?,
                Value::String(ref v) => self.write_string(v)?,
                Value::UnitVariant(ref v) => self.write_unit_variant(v)?,
                _ => unreachable!(),
            }

            if idx != last {
                self.out.push_str(", ");
            }
        }

        self.out.push_str("]\n\n");

        Ok(())
    }

    fn serialize_map(&mut self, map: &'v IndexMap<String, ValueCell>) -> Result<()> {
        for (k, v) in map {
            self.path.push_map_key(k);
            self.serialize(v)?;
            self.path.pop();
        }

        Ok(())
    }

    fn serialize_struct(&mut self, map: &'v IndexMap<String, ValueCell>) -> Result<()> {
        for (k, v) in map {
            validate_ident(k)?;
            self.path.push_struct_field_name(k);
            self.serialize(v)?;
            self.path.pop();
        }

        Ok(())
    }

    fn serialize_variant(&mut self, variant: &'v str, v: &'v ValueCell) -> Result<()> {
        validate_ident(variant)?;
        self.path.push_variant_name(variant);
        self.serialize(v)?;
        self.path.pop();

        Ok(())
    }

    #[inline]
    fn write_rhs_infallible(&mut self, writer: impl Fn(&mut KonfigSerializer)) -> Result<()> {
        self.write_rhs(|s| {
            writer(s);
            Ok(())
        })
    }

    fn write_rhs(&mut self, writer: impl Fn(&mut KonfigSerializer) -> Result<()>) -> Result<()> {
        self.path.write(&mut self.out).map_err(Error::custom)?;
        self.out.push_str(" = ");

        writer(self)?;

        // NOTE: always write expression separator. It' easier to trim it on completion than
        // any other approach requiring to track whether where will be more values or docs
        // serialized.
        self.out.push_str("\n\n");

        Ok(())
    }
}

fn is_all_primitive(seq: &[ValueCell]) -> bool {
    if seq.is_empty() {
        return true;
    }

    seq.iter().all(|v| match **v {
        Value::Null
        | Value::Bool(_)
        | Value::Float(_)
        | Value::Int(_)
        | Value::UInt(_)
        | Value::String(_)
        | Value::UnitVariant(_) => true,
        Value::Map(_) | Value::Struct(_) | Value::Sequence(_) | Value::Variant(_, _) => false,
    })
}

fn validate_ident(ident: &str) -> Result<()> {
    let mut chars = ident.chars();
    let first_ok = chars.next().map(char::is_alphabetic).unwrap_or_default();
    let rest_ok = chars.all(|c| c.is_ascii_alphanumeric() || c == '_');

    if first_ok && rest_ok {
        Ok(())
    } else {
        Err(Error::InvalidFieldNameOrEnumVariant(ident.to_string()))
    }
}
