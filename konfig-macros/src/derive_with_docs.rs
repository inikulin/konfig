use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::visit::Visit;
use syn::{
    Attribute, DataEnum, DataStruct, DeriveInput, Expr, ExprLit, Fields, FieldsNamed,
    FieldsUnnamed, Ident, Index, Lit, LitStr, Meta, MetaList, MetaNameValue, Variant,
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

#[derive(Default)]
struct AddDocsBodyCodegen(TokenStream2);

impl AddDocsBodyCodegen {
    fn expand(input: &DeriveInput) -> TokenStream2 {
        let mut codegen = Self::default();

        codegen.visit_derive_input(input);

        codegen.0
    }
}

impl Visit<'_> for AddDocsBodyCodegen {
    fn visit_data_struct(&mut self, data_struct: &DataStruct) {
        self.0 = StructAddDocsBodyCodegen::expand(data_struct);
    }

    fn visit_data_enum(&mut self, data_enum: &DataEnum) {
        let match_arms = VariantsMatchArmsCodegen::expand(data_enum);

        self.0 = quote! {
            match self {
                #match_arms
            }
        };
    }
}

#[derive(Default)]
struct StructAddDocsBodyCodegen(TokenStream2);

impl StructAddDocsBodyCodegen {
    fn expand(data_struct: &DataStruct) -> TokenStream2 {
        let mut codegen = Self::default();

        codegen.visit_data_struct(data_struct);

        codegen.0
    }
}

impl Visit<'_> for StructAddDocsBodyCodegen {
    fn visit_fields_unnamed(&mut self, fields: &FieldsUnnamed) {
        for (idx, field) in fields.unnamed.iter().enumerate() {
            let (docs, cfg_attrs) = extract_docs_and_cfg_attrs(&field.attrs);

            let idx_field = Index {
                index: idx.try_into().unwrap(),
                span: Span::call_site(),
            };

            self.0.extend(quote! {
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
        self.0 = expand_fields_named(fields, true);
    }
}

#[derive(Default)]
struct VariantsMatchArmsCodegen(TokenStream2);

impl VariantsMatchArmsCodegen {
    fn expand(data_enum: &DataEnum) -> TokenStream2 {
        let mut codegen = Self::default();

        codegen.visit_data_enum(data_enum);

        codegen.0
    }
}

impl Visit<'_> for VariantsMatchArmsCodegen {
    fn visit_variant(&mut self, variant: &Variant) {
        let name = &variant.ident;
        let span = name.span();
        let (docs, cfg_attrs) = extract_docs_and_cfg_attrs(&variant.attrs);

        if let Fields::Unit = &variant.fields {
            self.0.extend(quote_spanned! { span =>
                #(#cfg_attrs)*
                Self::#name => (),
            });

            return;
        }

        let name_str = name.to_string();
        let match_pattern = VariantMatchPatternCodegen::expand(&variant.fields);
        let fields_docs = VariantFieldsDocsCodegen::expand(&variant.fields);

        self.0.extend(quote_spanned! { span =>
            #(#cfg_attrs)*
            Self::#name #match_pattern => {
                path.push_variant_name(#name_str);
                #docs
                #fields_docs
                path.pop();
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

#[derive(Default)]
struct VariantFieldsDocsCodegen(TokenStream2);

impl VariantFieldsDocsCodegen {
    fn expand(fields: &Fields) -> TokenStream2 {
        let mut codegen = Self::default();

        codegen.visit_fields(fields);

        codegen.0
    }
}

impl Visit<'_> for VariantFieldsDocsCodegen {
    fn visit_fields_unnamed(&mut self, fields: &FieldsUnnamed) {
        let is_newtype_variant = fields.unnamed.len() == 1;

        if is_newtype_variant {
            let attrs = &fields.unnamed.first().unwrap().attrs;
            let (docs, cfg_attrs) = extract_docs_and_cfg_attrs(attrs);
            let field_name = tuple_enum_variant_field_name(0);

            self.0 = quote! {
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

            self.0.extend(quote! {
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
        self.0 = expand_fields_named(fields, false);
    }
}

fn expand_fields_named(fields: &FieldsNamed, is_struct_fields: bool) -> TokenStream2 {
    let mut expanded = quote! {};

    let maybe_self = if is_struct_fields {
        quote! { self. }
    } else {
        quote! {}
    };

    for field in &fields.named {
        let name = field.ident.as_ref().unwrap();
        let span = name.span();
        let name_str = LitStr::new(&name.to_string(), span);
        let (docs, cfg_attrs) = extract_docs_and_cfg_attrs(&field.attrs);

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
        quote! {}
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
}
