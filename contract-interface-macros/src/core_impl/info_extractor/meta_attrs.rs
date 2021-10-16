use crate::error;
use darling::ToTokens;
use quote::quote;

pub fn meta_attrs<Arg: darling::FromMeta>(
    attrs: &[syn::Attribute],
    additional_attrs: Vec<syn::NestedMeta>,
    name: &str,
) -> error::Result<(Arg, Vec<syn::Attribute>)> {
    let (attrs, forward_attrs) = internal_meta_attrs(attrs, name);
    let mut attrs = attrs?;
    attrs.extend(additional_attrs);
    Ok((Arg::from_list(&attrs)?, forward_attrs))
}

fn internal_meta_attrs(
    attrs: &[syn::Attribute],
    name: &str,
) -> (syn::Result<Vec<syn::NestedMeta>>, Vec<syn::Attribute>) {
    let (correct_name, remaining): (Vec<_>, Vec<_>) = attrs
        .into_iter()
        .cloned()
        .partition(|attr| attr.path.is_ident(name));

    let correct_name = correct_name
        .into_iter()
        .map(|a| a.parse_meta().map(syn::NestedMeta::from))
        .collect::<Result<_, _>>();

    (correct_name, remaining)
}
