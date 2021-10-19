use crate::error;
use darling::ToTokens;
use quote::quote;

pub fn meta_attrs<Arg: darling::FromMeta>(
    attrs: &[syn::Attribute],
    additional_attrs: Vec<syn::NestedMeta>,
    name: &str,
) -> error::Result<(Arg, Vec<syn::Attribute>)> {
    let (attrs, forward_attrs) = partition_attrs(attrs, name);
    let mut attrs = into_nested_meta_attrs(attrs)?;
    attrs.extend(additional_attrs);
    Ok((Arg::from_list(&attrs)?, forward_attrs))
}

pub fn partition_attrs(
    attrs: &[syn::Attribute],
    name: &str,
) -> (Vec<syn::Attribute>, Vec<syn::Attribute>) {
    let (correct_name, remaining): (Vec<_>, Vec<_>) = attrs
        .iter()
        .cloned()
        .partition(|attr| attr.path.is_ident(name));
    (correct_name, remaining)
}

pub fn into_meta_attrs(attrs: Vec<syn::Attribute>) -> syn::Result<Vec<syn::Meta>> {
    attrs
        .into_iter()
        .map(|a| a.parse_meta())
        .collect::<Result<_, _>>()
}

/// For `syn::Meta` elements, only allows `syn::Meta::List` ones and
/// grab the `syn::NestedMeta` inside of them.
pub fn remove_first_layer(
    metas: Vec<syn::Meta>,
    expected_ident: &str,
) -> syn::Result<Vec<syn::NestedMeta>> {
    use syn::spanned::Spanned;
    let mut nested = vec![];
    for m in metas {
        match m {
            syn::Meta::List(l) => {
                if !l.path.is_ident(expected_ident) {
                    return Err(syn::Error::new(
                        l.path.span(),
                        format!("Unexpected identifier. Expecting {}.", expected_ident),
                    ));
                }
                nested.extend(l.nested.into_iter().collect::<Vec<_>>())
            }
            syn::Meta::Path(p) => {
                return Err(syn::Error::new(
                    p.span(),
                    "Unexpected Meta::Path. Expecting Meta::List.",
                ));
            }
            syn::Meta::NameValue(n) => {
                return Err(syn::Error::new(
                    n.span(),
                    "Unexpected Meta::NameValue. Expecting Meta::List.",
                ));
            }
        }
    }
    Ok(nested)
}

pub fn into_nested_meta_attrs(attrs: Vec<syn::Attribute>) -> syn::Result<Vec<syn::NestedMeta>> {
    attrs
        .into_iter()
        .map(|a| a.parse_meta().map(syn::NestedMeta::from))
        .collect::<Result<_, _>>()
}
