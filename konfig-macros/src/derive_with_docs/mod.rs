mod serde_attrs;

use self::serde_attrs::{SerdeAttributesInfo, SerdeContainerAttributesInfo};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::visit::Visit;
use syn::{
    Attribute, DataEnum, DataStruct, DeriveInput, Expr, ExprLit, Fields, FieldsNamed,
    FieldsUnnamed, Ident, Index, Lit, Meta, MetaList, MetaNameValue, Variant,
};

#[derive(Default)]
pub(crate) struct ImplCodegen(TokenStream2);

impl ImplCodegen {
    pub(crate) fn expand(input: DeriveInput) -> TokenStream2 {
        let mut codegen = Self::default();

        codegen.visit_derive_input(&input);

        codegen.0
    }
}

impl Visit<'_> for ImplCodegen {
    fn visit_derive_input(&mut self, input: &DeriveInput) {
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let name = &input.ident;
        let span = name.span();
        let body = AddDocsBodyCodegen::expand(input);

        self.0 = quote_spanned! { span =>
            #[automatically_derived]
            impl #impl_generics konfig::WithDocs for #name #ty_generics #where_clause {
                fn add_docs(
                    &self,
                    path: &mut konfig::value::Path<'static, ()>,
                    docs: &mut std::collections::HashMap<Vec<konfig::value::PathItem<'static>>, String>,
                ) -> konfig::Result<()> {
                    #body

                    Ok(())
                }
            }
        };
    }
}

struct AddDocsBodyCodegen {
    out: TokenStream2,
    serde_container_attrs: SerdeContainerAttributesInfo,
}

impl AddDocsBodyCodegen {
    fn expand(input: &DeriveInput) -> TokenStream2 {
        let serde_container_attrs = SerdeContainerAttributesInfo::from(&input.attrs);

        let mut codegen = Self {
            out: TokenStream2::default(),
            serde_container_attrs,
        };

        codegen.visit_derive_input(input);

        codegen.out
    }
}

impl Visit<'_> for AddDocsBodyCodegen {
    fn visit_data_struct(&mut self, data_struct: &DataStruct) {
        self.out = StructAddDocsBodyCodegen::expand(data_struct, &self.serde_container_attrs);
    }

    fn visit_data_enum(&mut self, data_enum: &DataEnum) {
        let match_arms = VariantsMatchArmsCodegen::expand(data_enum, &self.serde_container_attrs);

        self.out = quote! {
            match self {
                #match_arms
            }
        };
    }
}

struct StructAddDocsBodyCodegen<'a> {
    out: TokenStream2,
    serde_container_attrs: &'a SerdeContainerAttributesInfo,
}

impl<'a> StructAddDocsBodyCodegen<'a> {
    fn expand(
        data_struct: &DataStruct,
        serde_container_attrs: &'a SerdeContainerAttributesInfo,
    ) -> TokenStream2 {
        let mut codegen = Self {
            out: TokenStream2::default(),
            serde_container_attrs,
        };

        codegen.visit_data_struct(data_struct);

        codegen.out
    }
}

impl Visit<'_> for StructAddDocsBodyCodegen<'_> {
    fn visit_fields_unnamed(&mut self, fields: &FieldsUnnamed) {
        for (idx, field) in fields.unnamed.iter().enumerate() {
            let (docs, cfg_attrs) = extract_docs_and_cfg_attrs(&field.attrs);

            let idx_field = Index {
                index: idx.try_into().unwrap(),
                span: Span::call_site(),
            };

            self.out.extend(quote! {
                #(#cfg_attrs)*
                {
                    path.push_sequence_index(#idx);
                    #docs
                    self.#idx_field.add_docs(path, docs)?;
                    path.pop();
                }
            });
        }
    }

    fn visit_fields_named(&mut self, fields: &FieldsNamed) {
        self.out = expand_fields_named(fields, true, self.serde_container_attrs);
    }
}

struct VariantsMatchArmsCodegen<'a> {
    out: TokenStream2,
    serde_container_attrs: &'a SerdeContainerAttributesInfo,
}

impl<'a> VariantsMatchArmsCodegen<'a> {
    fn expand(
        data_enum: &DataEnum,
        serde_container_attrs: &'a SerdeContainerAttributesInfo,
    ) -> TokenStream2 {
        let mut codegen = Self {
            out: TokenStream2::default(),
            serde_container_attrs,
        };

        codegen.visit_data_enum(data_enum);

        codegen.out
    }
}

impl Visit<'_> for VariantsMatchArmsCodegen<'_> {
    fn visit_variant(&mut self, variant: &Variant) {
        let name = &variant.ident;
        let span = name.span();
        let (docs, cfg_attrs) = extract_docs_and_cfg_attrs(&variant.attrs);

        if let Fields::Unit = &variant.fields {
            self.out.extend(quote_spanned! { span =>
                #(#cfg_attrs)*
                Self::#name => (),
            });

            return;
        }

        let match_pattern = VariantMatchPatternCodegen::expand(&variant.fields);

        let serde_container_attrs = SerdeContainerAttributesInfo::from(&variant.attrs);
        let fields_docs = VariantFieldsDocsCodegen::expand(&variant.fields, &serde_container_attrs);

        let mut body = quote! {
            #docs
            #fields_docs
        };

        if !self.serde_container_attrs.untagged() {
            let name_str = SerdeAttributesInfo::from(&variant.attrs).maybe_rename(
                self.serde_container_attrs
                    .maybe_rename_variant(name.to_string()),
            );

            body = quote! {
                path.push_variant_name(#name_str);
                #body
                path.pop();
            };
        }

        self.out.extend(quote_spanned! { span =>
            #(#cfg_attrs)*
            Self::#name #match_pattern => {
                #body
            }
        });
    }
}

#[derive(Default)]
struct VariantMatchPatternCodegen(TokenStream2);

impl VariantMatchPatternCodegen {
    fn expand(fields: &Fields) -> TokenStream2 {
        let mut codegen = Self::default();

        codegen.visit_fields(fields);

        codegen.0
    }
}

impl Visit<'_> for VariantMatchPatternCodegen {
    fn visit_fields_unnamed(&mut self, fields: &FieldsUnnamed) {
        let field_names = (0..fields.unnamed.len()).map(tuple_enum_variant_field_name);

        self.0 = quote! { ( #(#field_names),* ) }
    }

    fn visit_fields_named(&mut self, fields: &FieldsNamed) {
        let field_names = fields
            .named
            .iter()
            .map(|field| field.ident.as_ref().unwrap());

        self.0 = quote! { { #(#field_names),* } }
    }
}

struct VariantFieldsDocsCodegen<'a> {
    out: TokenStream2,
    serde_container_attrs: &'a SerdeContainerAttributesInfo,
}

impl<'a> VariantFieldsDocsCodegen<'a> {
    fn expand(
        fields: &Fields,
        serde_container_attrs: &'a SerdeContainerAttributesInfo,
    ) -> TokenStream2 {
        let mut codegen = Self {
            out: TokenStream2::default(),
            serde_container_attrs,
        };

        codegen.visit_fields(fields);

        codegen.out
    }
}

impl Visit<'_> for VariantFieldsDocsCodegen<'_> {
    fn visit_fields_unnamed(&mut self, fields: &FieldsUnnamed) {
        let is_newtype_variant = fields.unnamed.len() == 1;

        if is_newtype_variant {
            let attrs = &fields.unnamed.first().unwrap().attrs;
            let (docs, cfg_attrs) = extract_docs_and_cfg_attrs(attrs);
            let field_name = tuple_enum_variant_field_name(0);

            self.out = quote! {
                #(#cfg_attrs)*
                {
                    #docs
                    #field_name.add_docs(path, docs)?;
                }
            };

            return;
        }

        for (idx, field) in fields.unnamed.iter().enumerate() {
            let (docs, cfg_attrs) = extract_docs_and_cfg_attrs(&field.attrs);
            let field_name = tuple_enum_variant_field_name(idx);

            self.out.extend(quote! {
                #(#cfg_attrs)*
                {
                    path.push_sequence_index(#idx);
                    #docs
                    #field_name.add_docs(path, docs)?;
                    path.pop();
                }
            });
        }
    }

    fn visit_fields_named(&mut self, fields: &FieldsNamed) {
        self.out = expand_fields_named(fields, false, self.serde_container_attrs);
    }
}

fn expand_fields_named(
    fields: &FieldsNamed,
    is_struct_fields: bool,
    serde_container_attrs: &SerdeContainerAttributesInfo,
) -> TokenStream2 {
    let mut expanded = TokenStream2::default();

    let maybe_self = if is_struct_fields {
        quote! { self. }
    } else {
        TokenStream2::default()
    };

    for field in &fields.named {
        let name = field.ident.as_ref().unwrap();
        let span = name.span();
        let (docs, cfg_attrs) = extract_docs_and_cfg_attrs(&field.attrs);

        let name_str = SerdeAttributesInfo::from(&field.attrs)
            .maybe_rename(serde_container_attrs.maybe_rename_field(name.to_string()));

        expanded.extend(quote_spanned! { span =>
            #(#cfg_attrs)*
            {
                path.push_struct_field_name(#name_str);
                #docs
                #maybe_self #name.add_docs(path, docs)?;
                path.pop();
            }
        });
    }

    expanded
}

fn tuple_enum_variant_field_name(idx: usize) -> Ident {
    Ident::new(&format!("n{idx}"), Span::call_site())
}

fn extract_docs_and_cfg_attrs(attrs: &[Attribute]) -> (TokenStream2, Vec<&Attribute>) {
    let mut docs = vec![];
    let mut cfg_attrs = vec![];

    for attr in attrs {
        match &attr.meta {
            Meta::NameValue(MetaNameValue {
                path,
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit_str),
                        ..
                    }),
                ..
            }) if path.is_ident("doc") => docs.push(lit_str.value()),
            Meta::List(MetaList { path, .. }) if path.is_ident("cfg") => cfg_attrs.push(attr),
            _ => (),
        }
    }

    let docs = if docs.is_empty() {
        TokenStream2::default()
    } else {
        let docs = docs.join("\n");

        quote! { docs.insert(path.items().to_vec(), #docs.to_string()); }
    };

    (docs, cfg_attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_struct() {
        let input: DeriveInput = syn::parse_quote! {
            struct FooBar<T> where T: Copy {
                /// Field `foo`.
                ///
                /// Some description.
                foo: usize,
                /// Field `bar`.
                bar: Option<T>,
                // This is not a doc comment, so will be skipped.
                baz: String,
                /// Field `qux`.
                #[cfg(test)]
                qux: String
            }
        };

        let actual = ImplCodegen::expand(input);

        let expected: TokenStream2 = syn::parse_quote! {
            #[automatically_derived]
            impl<T> konfig::WithDocs for FooBar<T> where T: Copy {
                fn add_docs(
                    &self,
                    path: &mut konfig::value::Path<'static, ()>,
                    docs: &mut std::collections::HashMap<Vec<konfig::value::PathItem<'static>>, String>,
                ) -> konfig::Result<()> {
                    {
                        path.push_struct_field_name("foo");
                        docs.insert(
                            path.items().to_vec(),
                            " Field `foo`.\n\n Some description.".to_string()
                        );
                        self.foo.add_docs(path, docs)?;
                        path.pop();
                    }
                    {
                        path.push_struct_field_name("bar");
                        docs.insert(path.items().to_vec(), " Field `bar`.".to_string());
                        self.bar.add_docs(path, docs)?;
                        path.pop();
                    }
                    {
                        path.push_struct_field_name("baz");
                        self.baz.add_docs(path, docs)?;
                        path.pop();
                    }
                    #[cfg(test)]
                    {
                        path.push_struct_field_name("qux");
                        docs.insert(path.items().to_vec(), " Field `qux`.".to_string());
                        self.qux.add_docs(path, docs)?;
                        path.pop();
                    }

                    Ok(())
                }
            }
        };

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn expand_tuple_struct() {
        let input: DeriveInput = syn::parse_quote! {
            struct FooBar<T>(
                /// Field `foo`.
                ///
                /// Some description.
                usize,
                /// Field `bar`.
                Option<T>,
                // This is not a doc comment, so will be skipped.
                String,
                /// Field `qux`.
                #[cfg(test)]
                String
            ) where T: Copy;
        };

        let actual = ImplCodegen::expand(input);

        let expected: TokenStream2 = syn::parse_quote! {
            #[automatically_derived]
            impl<T> konfig::WithDocs for FooBar<T> where T: Copy {
                fn add_docs(
                    &self,
                    path: &mut konfig::value::Path<'static, ()>,
                    docs: &mut std::collections::HashMap<Vec<konfig::value::PathItem<'static>>, String>,
                ) -> konfig::Result<()> {
                    {
                        path.push_sequence_index(0usize);
                        docs.insert(
                            path.items().to_vec(),
                            " Field `foo`.\n\n Some description.".to_string()
                        );
                        self.0.add_docs(path, docs)?;
                        path.pop();
                    }
                    {
                        path.push_sequence_index(1usize);
                        docs.insert(path.items().to_vec(), " Field `bar`.".to_string());
                        self.1.add_docs(path, docs)?;
                        path.pop();
                    }
                    {
                        path.push_sequence_index(2usize);
                        self.2.add_docs(path, docs)?;
                        path.pop();
                    }
                    #[cfg(test)]
                    {
                        path.push_sequence_index(3usize);
                        docs.insert(path.items().to_vec(), " Field `qux`.".to_string());
                        self.3.add_docs(path, docs)?;
                        path.pop();
                    }

                    Ok(())
                }
            }
        };

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn expand_unit_struct() {
        let input: DeriveInput = syn::parse_quote! {
            struct FooBar<T> where T: Copy;
        };

        let actual = ImplCodegen::expand(input);

        let expected: TokenStream2 = syn::parse_quote! {
            #[automatically_derived]
            impl<T> konfig::WithDocs for FooBar<T> where T: Copy {
                fn add_docs(
                    &self,
                    path: &mut konfig::value::Path<'static, ()>,
                    docs: &mut std::collections::HashMap<Vec<konfig::value::PathItem<'static>>, String>,
                ) -> konfig::Result<()> {
                    Ok(())
                }
            }
        };

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn expand_enum() {
        let input: DeriveInput = syn::parse_quote! {
            enum FooBar<T> where T: Copy {
                /// UnitVariant docs.
                UnitVariant,

                /// NewtypeVariant docs.
                NewtypeVariant(String),

                /// TupleVariant docs.
                TupleVariant(
                    /// Field `foo`.
                    ///
                    /// Some description.
                    usize,
                    /// Field `bar`.
                    Option<T>,
                    // This is not a doc comment, so will be skipped.
                    String,
                    /// Field `qux`.
                    #[cfg(test)]
                    String
                ),

                /// StructVariant docs.
                StructVariant {
                    /// Field `foo`.
                    ///
                    /// Some description.
                    foo: usize,
                    /// Field `bar`.
                    bar: Option<T>,
                    // This is not a doc comment, so will be skipped.
                    baz: String,
                    /// Field `qux`.
                    #[cfg(test)]
                    qux: String
                }
            }
        };

        let actual = ImplCodegen::expand(input);

        let expected: TokenStream2 = syn::parse_quote! {
            #[automatically_derived]
            impl<T> konfig::WithDocs for FooBar<T>
            where
                T: Copy
            {
                fn add_docs(
                    &self,
                    path: &mut konfig::value::Path<'static, ()>,
                    docs: &mut std::collections::HashMap<Vec<konfig::value::PathItem<'static>>, String>,
                ) -> konfig::Result<()> {
                    match self {
                        Self::UnitVariant => (),
                        Self::NewtypeVariant(n0) => {
                            path.push_variant_name("NewtypeVariant");
                            docs.insert(path.items().to_vec(), " NewtypeVariant docs.".to_string());
                            {
                                n0.add_docs(path, docs)?;
                            }
                            path.pop();
                        }
                        Self::TupleVariant(n0, n1, n2, n3) => {
                            path.push_variant_name("TupleVariant");
                            docs.insert(path.items().to_vec(), " TupleVariant docs.".to_string());
                            {
                                path.push_sequence_index(0usize);
                                docs.insert(
                                    path.items().to_vec(),
                                    " Field `foo`.\n\n Some description.".to_string()
                                );
                                n0.add_docs(path, docs)?;
                                path.pop();
                            }
                            {
                                path.push_sequence_index(1usize);
                                docs.insert(path.items().to_vec(), " Field `bar`.".to_string());
                                n1.add_docs(path, docs)?;
                                path.pop();
                            }
                            {
                                path.push_sequence_index(2usize);
                                n2.add_docs(path, docs)?;
                                path.pop();
                            }
                            #[cfg(test)]
                            {
                                path.push_sequence_index(3usize);
                                docs.insert(path.items().to_vec(), " Field `qux`.".to_string());
                                n3.add_docs(path, docs)?;
                                path.pop();
                            }
                            path.pop();
                        }
                        Self::StructVariant { foo, bar, baz, qux } => {
                            path.push_variant_name("StructVariant");
                            docs.insert(path.items().to_vec(), " StructVariant docs.".to_string());
                            {
                                path.push_struct_field_name("foo");
                                docs.insert(
                                    path.items().to_vec(),
                                    " Field `foo`.\n\n Some description.".to_string()
                                );
                                foo.add_docs(path, docs)?;
                                path.pop();
                            }
                            {
                                path.push_struct_field_name("bar");
                                docs.insert(path.items().to_vec(), " Field `bar`.".to_string());
                                bar.add_docs(path, docs)?;
                                path.pop();
                            }
                            {
                                path.push_struct_field_name("baz");
                                baz.add_docs(path, docs)?;
                                path.pop();
                            }
                            #[cfg(test)]
                            {
                                path.push_struct_field_name("qux");
                                docs.insert(path.items().to_vec(), " Field `qux`.".to_string());
                                qux.add_docs(path, docs)?;
                                path.pop();
                            }
                            path.pop();
                        }
                    }
                    Ok(())
                }
            }

        };

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn expand_struct_with_serde_rename() {
        let input: DeriveInput = syn::parse_quote! {
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct FooBar<T> where T: Copy {
                foo: usize,
                bar: Option<T>,
                #[serde(rename(serialize = "baz_renamed"))]
                baz: String,
                #[serde(rename = "qux_renamed")]
                qux: String
            }
        };

        let actual = ImplCodegen::expand(input);

        let expected: TokenStream2 = syn::parse_quote! {
            #[automatically_derived]
            impl<T> konfig::WithDocs for FooBar<T> where T: Copy {
                fn add_docs(
                    &self,
                    path: &mut konfig::value::Path<'static, ()>,
                    docs: &mut std::collections::HashMap<Vec<konfig::value::PathItem<'static>>, String>,
                ) -> konfig::Result<()> {
                    {
                        path.push_struct_field_name("FOO");
                        self.foo.add_docs(path, docs)?;
                        path.pop();
                    }
                    {
                        path.push_struct_field_name("BAR");
                        self.bar.add_docs(path, docs)?;
                        path.pop();
                    }
                    {
                        path.push_struct_field_name("baz_renamed");
                        self.baz.add_docs(path, docs)?;
                        path.pop();
                    }
                    {
                        path.push_struct_field_name("qux_renamed");
                        self.qux.add_docs(path, docs)?;
                        path.pop();
                    }

                    Ok(())
                }
            }
        };

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn expand_enum_with_serde_rename() {
        let input: DeriveInput = syn::parse_quote! {
            #[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
            enum FooBar<T> where T: Copy {
                #[serde(rename = "NewtypeVariant_renamed")]
                NewtypeVariant(String),

                #[serde(rename_all(serialize = "camelCase"))]
                StructVariant {
                    #[serde(rename = "foo_renamed")]
                    foo: usize,
                    bar_qux: Option<T>,
                }
            }
        };

        let actual = ImplCodegen::expand(input);

        let expected: TokenStream2 = syn::parse_quote! {
            #[automatically_derived]
            impl<T> konfig::WithDocs for FooBar<T>
            where
                T: Copy
            {
                fn add_docs(
                    &self,
                    path: &mut konfig::value::Path<'static, ()>,
                    docs: &mut std::collections::HashMap<Vec<konfig::value::PathItem<'static>>, String>,
                ) -> konfig::Result<()> {
                    match self {
                        Self::NewtypeVariant(n0) => {
                            path.push_variant_name("NewtypeVariant_renamed");
                            {
                                n0.add_docs(path, docs)?;
                            }
                            path.pop();
                        }
                        Self::StructVariant { foo, bar_qux } => {
                            path.push_variant_name("STRUCT_VARIANT");
                            {
                                path.push_struct_field_name("foo_renamed");
                                foo.add_docs(path, docs)?;
                                path.pop();
                            }
                            {
                                path.push_struct_field_name("barQux");
                                bar_qux.add_docs(path, docs)?;
                                path.pop();
                            }
                            path.pop();
                        }
                    }
                    Ok(())
                }
            }

        };

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn expand_enum_with_serde_untagged() {
        let input: DeriveInput = syn::parse_quote! {
            #[serde(untagged)]
            enum FooBar<T> where T: Copy {
                NewtypeVariant(String),
            }
        };

        let actual = ImplCodegen::expand(input);

        let expected: TokenStream2 = syn::parse_quote! {
            #[automatically_derived]
            impl<T> konfig::WithDocs for FooBar<T>
            where
                T: Copy
            {
                fn add_docs(
                    &self,
                    path: &mut konfig::value::Path<'static, ()>,
                    docs: &mut std::collections::HashMap<Vec<konfig::value::PathItem<'static>>, String>,
                ) -> konfig::Result<()> {
                    match self {
                        Self::NewtypeVariant(n0) => {
                            {
                                n0.add_docs(path, docs)?;
                            }
                        }
                    }
                    Ok(())
                }
            }

        };

        assert_eq!(actual.to_string(), expected.to_string());
    }
}
