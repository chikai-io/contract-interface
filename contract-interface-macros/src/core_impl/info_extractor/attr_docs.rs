use crate::error;

pub fn parse_attr_docs(attrs: &[syn::Attribute]) -> error::Result<Vec<syn::Lit>> {
    let mut docs = vec![];
    for attr in attrs {
        if !matches!(attr.style, syn::AttrStyle::Outer) {
            continue;
        }

        if attr.path.is_ident("doc") {
            match attr.parse_meta()? {
                syn::Meta::NameValue(mnv) => docs.push(mnv.lit),
                bad => {
                    return Err(syn::Error::new_spanned(bad, "unrecognized doc attribute").into())
                }
            };
        }
    }
    Ok(docs)
}
