use crate::error;
use darling::ToTokens;
use quote::quote;

pub fn meta_attrs<Arg: darling::FromMeta>(
    attrs: &[syn::Attribute],
    additional_attrs: Vec<syn::NestedMeta>,
    name: &str,
) -> error::Result<(Arg, Vec<syn::Attribute>)> {
    let (attrs, forward_attrs) = partition_attrs(attrs, name);
    let mut attrs = into_meta_attrs(attrs)?;
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

fn into_meta_attrs(attrs: Vec<syn::Attribute>) -> syn::Result<Vec<syn::NestedMeta>> {
    attrs
        .into_iter()
        .map(|a| a.parse_meta().map(syn::NestedMeta::from))
        .collect::<Result<_, _>>()
}
