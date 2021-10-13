use crate::info_extractor::item_impl_info_called_in::ItemImplInfo;
use quote::quote;
use syn::export::TokenStream2;
use syn::Ident;

impl ItemImplInfo {
    /// Generate the code that wraps
    pub fn wrapper_code(&self) -> TokenStream2 {
        let mut result = Vec::new();
        for (_ident, method) in &self.items.methods {
            result.extend(method.method_wrapper());
        }

        let original = &self.original;
        let struct_mod_name = &self.ident;
        let impl_docs = &self.docs;

        quote! {
            #(#[doc = #impl_docs])*
            pub mod #struct_mod_name {
                use super::*;

                #(#result)*
            }
        }
    }

    // TODO
    pub fn marshall_code(&self) -> TokenStream2 {
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
        quote! {
         #[cfg(not(target_arch = "wasm32"))]
         impl #name {
           #res
         }
        }
    }
}
