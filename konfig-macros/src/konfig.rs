use konfig_edit::serializer::components::write_float;
use konfig_edit::value::{Value, ValueCell};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{LitBool, LitFloat, LitInt, LitStr};

pub(crate) fn expand(arg: LitStr) -> TokenStream2 {
    let value = match konfig_edit::parser::parse(&arg.value()) {
        Ok(value) => value,
        Err(e) => return syn::Error::new(Span::call_site(), e).to_compile_error(),
    };

    gen_value_code(&value)
}

fn gen_value_code(value: &ValueCell) -> TokenStream2 {
    match *value.as_value() {
        Value::Null => quote! { Value::Null.into_cell() },
        Value::Bool(v) => {
            let v = LitBool::new(v, Span::call_site());

            quote! { Value::Bool(#v).into_cell() }
        }
        Value::Int(v) => {
            let v = LitInt::new(&v.to_string(), Span::call_site());

            quote! { Value::Int(#v).into_cell() }
        }
        Value::UInt(v) => {
            let v = LitInt::new(&v.to_string(), Span::call_site());

            quote! { Value::UInt(#v).into_cell() }
        }
        Value::Float(v) => {
            let mut v_str = String::with_capacity(16);

            // NOTE: ironically, default formatting of floats generate literals that can't
            // be parsed by the Rust compiler if exponent is quite big. Formatting in scientific
            // notation (`"{:+e}"`) would have helped, but leading `+` for positive floats is not
            // accepted by `syn`. Luckily, we already have serialization routine for floats.
            write_float(&mut v_str, v).unwrap();

            let v = LitFloat::new(&v_str, Span::call_site());

            quote! { Value::Float(#v).into_cell() }
        }
        Value::String(ref v) => {
            let v = LitStr::new(v, Span::call_site());

            quote! { Value::String(#v.to_string()).into_cell() }
        }
        Value::UnitVariant(ref v) => {
            let v = LitStr::new(v, Span::call_site());

            quote! { Value::UnitVariant(#v.to_string()).into_cell() }
        }
        Value::Sequence(ref v) => {
            let v: Vec<_> = v.iter().map(gen_value_code).collect();

            quote! { Value::Sequence(vec![ #(#v),* ]).into_cell() }
        }
        Value::Map(ref v) => {
            let v = gen_map_code(v.iter());

            quote! { Value::Map(#v).into_cell() }
        }
        Value::Struct(ref v) => {
            let v = gen_map_code(v.iter());

            quote! { Value::Struct(#v).into_cell() }
        }
        Value::Variant(ref n, ref v) => {
            let n = LitStr::new(n, Span::call_site());
            let v = gen_value_code(v);

            quote! { Value::Variant(#n.to_string(), #v).into_cell() }
        }
    }
}

fn gen_map_code<'i>(map: impl Iterator<Item = (&'i String, &'i ValueCell)>) -> TokenStream2 {
    let kv: Vec<_> = map
        .map(|(k, v)| {
            let k = LitStr::new(k, Span::call_site());
            let v = gen_value_code(v);

            quote! { (#k.to_string(), #v) }
        })
        .collect();

    quote! { [#(#kv),*].into_iter().collect() }
}
