use crate::error::Result;
use proc_macro2::Span;

const CRATE_NAME: &str = "contract_interface";

pub fn crate_name_str(name: &str) -> Result<String> {
    use proc_macro_crate::FoundCrate;
    let name = match proc_macro_crate::crate_name(name)
        .map_err(|e| syn::Error::new(Span::call_site(), e))?
    {
        FoundCrate::Itself => CRATE_NAME.into(),
        FoundCrate::Name(name) => name,
    };
    Ok(name)
}

pub fn crate_name(name: &str) -> Result<syn::Ident> {
    use proc_macro_crate::FoundCrate;
    let name = match proc_macro_crate::crate_name(name)
        .map_err(|e| syn::Error::new(Span::call_site(), e))?
    {
        FoundCrate::Itself => syn::Ident::new(CRATE_NAME, Span::call_site()),
        FoundCrate::Name(name) => syn::Ident::new(&name, Span::call_site()),
    };
    Ok(name)
}
