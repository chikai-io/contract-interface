use crate::error;
use crate::info_extractor::item_impl_info::ItemImplInfo;
use quote::quote;
use syn::export::TokenStream2;
use syn::Ident;

impl ItemImplInfo {
    /// Generate the code that wraps
    pub fn wrapper_code(&self) -> error::Result<TokenStream2> {
        let mut result = Vec::new();
        let mut original = self.original.clone();
        original.attrs.clear();
        let forward_attrs = &self.forward_attrs;

        for (original_method_ident, method) in &self.items.methods {
            result.extend(method.method_wrapper(original_method_ident, self)?);
        }

        let struct_mod_name = &self.attrs.module_name;
        let self_ty = &self.self_ty;
        let original_doc_msg = format!(" For usage as a contract, see [`{}`].", struct_mod_name);

        let doc_generated = if let Some(trait_path) = &self.trait_path {
            // https://github.com/rust-lang/rust/issues/74563
            //
            // TODO: currently it's not possible to link directly to
            // the implementation's documentation itself, so both
            // the trait and the struct are referred
            format!(
                " Generated code based on an implementation of [`{}`] for [`{}`].",
                quote! {#trait_path},
                quote! {#self_ty}
            )
        } else {
            format!(
                " Generated code based on an implementation for [`{}`].",
                quote! {#self_ty}
            )
        };

        let doc_attrs = &self.doc_attrs;

        Ok(quote! {
            #(#doc_attrs)*
            #[doc = ""]
            #[doc = #original_doc_msg]
            #(#forward_attrs)*
            #original

            #[doc = #doc_generated]
            #[doc = ""]
            #(#doc_attrs)*
            pub mod #struct_mod_name {
                use super::*;

                #(#result)*
            }
        })
    }

    // TODO
    pub fn marshall_code(&self) -> error::Result<TokenStream2> {
        use quote::{format_ident, quote, ToTokens};
        let orig_name = self.self_ty.clone().into_token_stream();
        let mut name = quote! {Contract};
        if let Ok(input) = syn::parse::<Ident>(orig_name.into()) {
            let new_name = format_ident!("{}Contract", input);
            name = quote! {#new_name};
        };
        let mut res = TokenStream2::new();
        for (_ident, method) in &self.items.methods {
            res.extend(method.marshal_method());
        }
        Ok(quote! {
         #[cfg(not(target_arch = "wasm32"))]
         impl #name {
           #res
         }
        })
    }
}
