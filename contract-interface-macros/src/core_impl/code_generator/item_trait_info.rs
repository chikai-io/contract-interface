use crate::core_impl::info_extractor::item_trait_info::ItemTraitInfo;
use crate::error;
use quote::quote;
use syn::export::TokenStream2;

impl ItemTraitInfo {
    /// Generate code that wrapps external calls.
    pub fn wrapped_module(&self) -> error::Result<TokenStream2> {
        let mut result = TokenStream2::new();
        let mut original = self.original.clone();
        original.attrs.clear();
        let non_contract_attrs = &self.non_contract_attrs;

        for (original_method_ident, method) in &self.items.methods {
            result.extend(method.method_wrapper(original_method_ident, self));
        }
        let trait_mod_name = &self.attrs.module_name;
        let trait_doc_attrs = &self.doc_attrs;
        #[allow(unused_variables)]
        let original_trait_ident = &self.original_ident;

        let original_doc_msg = format!(" For usage as a contract, see [`{}`].", trait_mod_name);
        let mod_doc_msg = format!(" Generated code based on [`{}`].", original_trait_ident);

        Ok(quote! {
            #(#trait_doc_attrs)*
            #[doc = ""]
            #[doc = #original_doc_msg]
            #(#non_contract_attrs)*
            #original

            #[doc = #mod_doc_msg]
            #[doc = ""]
            #(#trait_doc_attrs)*
            pub mod #trait_mod_name {
                use super::*;

                #result
            }
        })
    }
}
