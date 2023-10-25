use darling::ast::NestedMeta;
use darling::{FromAttributes, FromMeta};
use serde_derive_internals::attr::RenameRule;
use syn::Attribute;

#[derive(FromAttributes, Debug, PartialEq, Default)]
#[darling(attributes(serde), allow_unknown_fields)]
pub(crate) struct SerdeContainerAttributesInfo {
    rename_all: Option<Rename>,
    untagged: Option<bool>,
}

impl From<&Vec<Attribute>> for SerdeContainerAttributesInfo {
    fn from(attrs: &Vec<Attribute>) -> Self {
        SerdeContainerAttributesInfo::from_attributes(attrs).unwrap_or_default()
    }
}

impl SerdeContainerAttributesInfo {
    pub(crate) fn untagged(&self) -> bool {
        self.untagged.unwrap_or(false)
    }

    pub(crate) fn maybe_rename_field(&self, name: String) -> String {
        match self.rename_rule() {
            Some(rule) => rule.apply_to_field(&name),
            None => name,
        }
    }

    pub(crate) fn maybe_rename_variant(&self, name: String) -> String {
        match self.rename_rule() {
            Some(rule) => rule.apply_to_variant(&name),
            None => name,
        }
    }

    fn rename_rule(&self) -> Option<RenameRule> {
        self.rename_all
            .as_ref()
            .and_then(|r| RenameRule::from_str(&r.0).ok())
    }
}

#[derive(FromAttributes, Debug, PartialEq, Default)]
#[darling(attributes(serde), allow_unknown_fields, forward_attrs(cfg, allow))]
pub(crate) struct SerdeAttributesInfo {
    #[darling(rename = "rename")]
    rename: Option<Rename>,
}

impl From<&Vec<Attribute>> for SerdeAttributesInfo {
    fn from(attrs: &Vec<Attribute>) -> Self {
        SerdeAttributesInfo::from_attributes(attrs).unwrap_or_default()
    }
}

impl SerdeAttributesInfo {
    pub(crate) fn maybe_rename(&self, name: String) -> String {
        match self.rename {
            Some(ref rename) => rename.0.to_string(),
            None => name,
        }
    }
}

#[derive(FromMeta)]
#[darling(allow_unknown_fields)]
struct RenameSerialize {
    serialize: String,
}

#[derive(Debug, PartialEq)]
struct Rename(String);

impl FromMeta for Rename {
    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Self(value.to_string()))
    }

    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        RenameSerialize::from_list(items).map(|r| Self(r.serialize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::ItemEnum;

    #[test]
    fn parse_container_attrs() {
        let src: ItemEnum = syn::parse_quote! {
            #[serde(untagged, rename_all = "foo", foobar = "baz")]
            enum FooBar {}
        };

        let attr_info = SerdeContainerAttributesInfo::from(&src.attrs);

        assert_eq!(
            attr_info,
            SerdeContainerAttributesInfo {
                rename_all: Some(Rename("foo".into())),
                untagged: Some(true)
            }
        );

        let src: ItemEnum = syn::parse_quote! {
            #[serde(untagged, foobar = "baz")]
            #[serde(rename_all(serialize = "foo"))]
            enum FooBar {}
        };

        let attr_info = SerdeContainerAttributesInfo::from(&src.attrs);

        assert_eq!(
            attr_info,
            SerdeContainerAttributesInfo {
                rename_all: Some(Rename("foo".into())),
                untagged: Some(true)
            }
        );
    }

    #[test]
    fn parse_attrs() {
        let src: ItemEnum = syn::parse_quote! {
            #[serde(rename = "foo", foobar = "baz")]
            enum FooBar {}
        };

        let attr_info = SerdeAttributesInfo::from(&src.attrs);

        assert_eq!(
            attr_info,
            SerdeAttributesInfo {
                rename: Some(Rename("foo".into())),
            }
        );

        let src: ItemEnum = syn::parse_quote! {
            #[serde(rename(serialize = "foo"))]
            enum FooBar {}
        };

        let attr_info = SerdeAttributesInfo::from(&src.attrs);

        assert_eq!(
            attr_info,
            SerdeAttributesInfo {
                rename: Some(Rename("foo".into())),
            }
        );
    }
}
