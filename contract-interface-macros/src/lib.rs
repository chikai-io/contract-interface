#![recursion_limit = "128"]
extern crate proc_macro;

mod core_impl;
mod crate_name;
mod error;
mod get_ident;
mod replace_ident;
mod replace_type_ident;

use self::core_impl::*;
pub(crate) use crate_name::{crate_name, crate_name_str};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

// TODO: describe all places where the attribute can be applied,
// their overall objectives/customization.
// TODO: add examples.
// TODO: possibly have a module for documentation-only.
//
/// When applied in a root item such as a trait or on an impl item,
/// this macro generates a `mod` containing contract-related information.
///
/// It also searches for more `#[contract]` attributes on inner items,
/// such as on methods or their arguments, which can configure the
/// generated code.
///
/// To see detailed information of the generated items, it is recommended
/// to generate and open the documentation of your project.
#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = syn::parse_macro_input!(attr as syn::AttributeArgs);
    match contract_internal(attr_args, item) {
        Ok(ok) => ok,
        Err(e) => e.into_token_stream(),
    }
}

fn contract_internal(
    attr_args: syn::AttributeArgs,
    item: TokenStream,
) -> error::Result<TokenStream> {
    // attached on `trait Trait {}`
    if let Ok(mut item_trait) = syn::parse::<syn::ItemTrait>(item.clone()) {
        let item_trait_info =
            info_extractor::item_trait_info::ItemTraitInfo::new(&mut item_trait, attr_args)?;
        Ok(item_trait_info.wrapped_module()?.into())
    }
    // attached on `impl Trait for Struct {}`
    else if let Ok(mut item_impl) = syn::parse::<syn::ItemImpl>(item) {
        let item_impl_info =
            info_extractor::item_impl_info::ItemImplInfo::new(&mut item_impl, attr_args)?;
        let generated_code = item_impl_info.wrapper_code()?;
        // Add helper type for simulation testing only if not wasm32
        // let marshalled_code = item_impl_info.marshall_code()?;
        Ok(TokenStream::from(quote! {
            // #item_impl
            #generated_code
            // #marshalled_code
        }))
    }
    // invalid root #[contract] attribute attachment
    else {
        Err(syn::Error::new(
            Span::call_site(),
            "`contract` can only be used on trait definitions or on implementations. Perhaps a `#[contract]` attribute is missing at the parent item?",
        )
        .into())
    }
}
