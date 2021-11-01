use crate::error;
use crate::info_extractor::item_impl_info::ItemImplInfo;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl ItemImplInfo {
    /// Generate the code that wraps
    pub fn wrapper_code(&self) -> error::Result<TokenStream2> {
        let internal_interface = crate::crate_name("contract-interface")?;

        let mut result = Vec::new();
        let mut original = self.original.clone();
        original.attrs.clear();
        let forward_attrs = &self.forward_attrs;

        // let impl_generic_lifetime_idents = &self.generics.lifetimes.keys().collect::<Vec<_>>();
        let impl_generic_type_idents = &self.generics.types.keys().collect::<Vec<_>>();
        let impl_generic_const_idents = &self.generics.consts.keys().collect::<Vec<_>>();

        let mut methods = Vec::new();
        let mut macro_method_params = Vec::new();
        let mut macro_methods = Vec::new();

        for (original_method_ident, method) in &self.items.methods {
            result.extend(method.method_wrapper(original_method_ident, self)?);

            // let method_generic_lifetime_idents = method.generics.lifetimes.keys().collect::<Vec<_>>();
            // let method_generic_lifetime_method_idents = &method_generic_lifetime_idents
            //     .iter()
            //     .map(|lt| {
            //         syn::Ident::new(
            //             &format!("{}_{}", original_method_ident, lt.ident),
            //             proc_macro2::Span::call_site(),
            //         )
            //     })
            //     .collect::<Vec<_>>();

            let method_generic_type_idents = method.generics.types.keys().collect::<Vec<_>>();
            let method_generic_type_method_idents = &method_generic_type_idents
                .iter()
                .map(|ty| {
                    syn::Ident::new(
                        &format!("{}_{}", original_method_ident, ty),
                        proc_macro2::Span::call_site(),
                    )
                })
                .collect::<Vec<_>>();

            let method_generic_const_idents = method.generics.consts.keys().collect::<Vec<_>>();
            let method_generic_const_method_idents = &method_generic_const_idents
                .iter()
                .map(|cst| {
                    syn::Ident::new(
                        &format!("{}_{}", original_method_ident, cst),
                        proc_macro2::Span::call_site(),
                    )
                })
                .collect::<Vec<_>>();

            // for (lt, mlt) in method_generic_lifetime_idents
            //     .iter()
            //     .zip(method_generic_lifetime_method_idents)
            // {
            //     macro_method_params.push(quote! {
            //         #original_method_ident <#lt> = $#mlt
            //     });
            // }
            for (ty, mty) in method_generic_type_idents
                .iter()
                .zip(method_generic_type_method_idents)
            {
                macro_method_params.push(quote! {
                    #original_method_ident <#ty> = $#mty:path
                });
            }
            for (cst, mcst) in method_generic_const_idents
                .iter()
                .zip(method_generic_const_method_idents)
            {
                macro_method_params.push(quote! {
                    #original_method_ident <#cst> = $#mcst:expr
                });
            }

            let receiver_kind = &method.inputs.receiver_kind.quote_trait_name();

            let macro_call = quote! {
                // TODO: consider adding arbitrary feature flag
                // TODO: add #[cfg(target_arch = "wasm32")]
                #[no_mangle]
                pub extern "C" fn #original_method_ident() {
                    use #internal_interface as _interface;
                    use #receiver_kind;
                    $($impl_mod::)*#original_method_ident::Serve::<
                        // #(#impl_generic_lifetime_idents,)*
                        // #($#method_generic_lifetime_method_idents,)*
                        #($#method_generic_type_method_idents,)*
                        #($#impl_generic_type_idents,)*
                        #($#impl_generic_const_idents,)*
                        #($#method_generic_const_method_idents,)*
                    >::extern_serve();
                }
            };
            macro_methods.push(macro_call);
            methods.push(original_method_ident.clone());
        }

        let struct_mod_name = &self.attrs.module_name;
        let struct_macro_name = &syn::Ident::new(
            &format!("extern_{}", struct_mod_name),
            proc_macro2::Span::call_site(),
        );
        let self_ty = &self.self_ty;
        let original_doc_msg = format!(" For usage as a contract, see [`{}`].", struct_mod_name);

        let doc_generated = if let Some(trait_path) = &self.trait_path {
            let mut trait_path_no_generics = trait_path.clone();
            let mut _last_segment =
                trait_path_no_generics
                    .segments
                    .iter_mut()
                    .rev()
                    .next()
                    .map(|s| {
                        s.arguments = syn::PathArguments::None;
                        s
                    });

            // https://github.com/rust-lang/rust/issues/74563
            //
            // TODO: currently it's not possible to link directly to
            // the implementation's documentation itself, so both
            // the trait and the struct are referred
            format!(
                " Generated code based on an implementation of [`{}`] for [`{}`].",
                quote! {#trait_path_no_generics},
                quote! {#self_ty}
            )
        } else {
            format!(
                " Generated code based on an implementation for [`{}`].",
                quote! {#self_ty}
            )
        };

        let doc_attrs = &self.doc_attrs;

        let macros = if self.attrs.serve {
            quote! {
                #[macro_use]
                mod exported_macro {
                    #[doc = #doc_generated]
                    #[doc = ""]
                    #[doc = " Generates `extern \"C\"` functions for the methods of this implementation."]
                    #[doc = ""]
                    #(#doc_attrs)*
                    #[macro_export]
                    macro_rules! #struct_macro_name {
                        (
                            impl_mod = $($impl_mod:ident)::*
                            // #(, <#impl_generic_lifetime_idents> = $#impl_generic_lifetime_idents:path)*
                            #(, <#impl_generic_type_idents> = $#impl_generic_type_idents:path)*
                            #(, <#impl_generic_const_idents> = $#impl_generic_const_idents:expr)*
                            #(, #macro_method_params)*
                        ) => {
                            #(#macro_methods)*
                        };
                    }
                }
            }
        } else {
            quote! {}
        };

        Ok(quote! {
            #(#doc_attrs)*
            #[doc = ""]
            #[doc = #original_doc_msg]
            #(#forward_attrs)*
            #original

            #[doc = #doc_generated]
            #[doc = ""]
            #(#doc_attrs)*
            #[macro_use]
            pub mod #struct_mod_name {
                use super::*;

                #macros

                #(#result)*

            }
        })
    }
}
