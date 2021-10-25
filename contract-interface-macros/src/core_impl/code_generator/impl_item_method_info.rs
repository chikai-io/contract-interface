use crate::core_impl::info_extractor::{
    attr_sig_info::AttrSigInfo, impl_item_method_info::ImplItemMethodInfo, InputStructType,
    MethodType, SerializerType,
};
use crate::error;
use crate::info_extractor::{inputs, item_impl_info::ItemImplInfo};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ReturnType, Signature};

impl ImplItemMethodInfo {
    /// Generate wrapper method for the given method of the contract.
    pub fn method_wrapper(
        &self,
        original_method_ident: &syn::Ident,
        impl_info: &ItemImplInfo,
    ) -> error::Result<TokenStream2> {
        let internal_interface = crate::crate_name("contract-interface")?;

        let self_ty = &impl_info.self_ty;
        let self_ty_str = format!("{}", quote!(#self_ty));

        let doc_generated = if let Some(trait_path) = &impl_info.trait_path {
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
                " Generated code based on the implementation of [`{}::{}()`] for [`{}`].",
                quote! {#trait_path_no_generics},
                quote! {#original_method_ident},
                quote! {#self_ty}
            )
        } else {
            format!(
                " Generated code based on the implementation for [`{}::{}()`].",
                quote! {#self_ty},
                quote! {#original_method_ident},
            )
        };

        // let mod_doc_msg = format!(
        //     " Generated code based on [`{}::{}()`].",
        //     &trait_info.original_ident, original_method_name
        // );

        let method_mod_name = &self.attrs.module_name;
        let attr_docs = &self.doc_attrs;

        // Ok(quote! {})

        let state_ty = &impl_info.self_ty;
        let res = if let Some(trait_path) = &impl_info.trait_path {
            let last_segment = if let Some(s) = trait_path.segments.iter().rev().next() {
                s
            } else {
                use syn::spanned::Spanned;
                return Err(syn::Error::new(
                    trait_path.span(),
                    "Could not find any segment for trait path.",
                )
                .into());
            };
            let last_segment_ident = &last_segment.ident;
            let mut trait_generics_lifetimes_idents = vec![];
            let mut trait_generics_type_or_const_idents = vec![];
            let mut trait_generics_const_exprs = vec![];
            match &last_segment.arguments {
                // no additional generics
                syn::PathArguments::None => {}
                // some generics. Needs to separate the lifetime from the rest,
                // because the state shall be palced as the first aprameter
                // right after the lifetimes
                syn::PathArguments::AngleBracketed(a) => {
                    for a in &a.args {
                        match a {
                            syn::GenericArgument::Lifetime(l) => {
                                trait_generics_lifetimes_idents.push(l);
                            }
                            // note: const idents may also fall in this
                            // arm
                            syn::GenericArgument::Type(t) => {
                                trait_generics_type_or_const_idents.push(t);
                            }
                            syn::GenericArgument::Const(c) => {
                                trait_generics_const_exprs.push(c);
                            }
                            // TODO: think/check if it should be implemented (I'm not sure)
                            // TODO: try to find an example that should be a valid implementation
                            // in normal rust
                            syn::GenericArgument::Binding(b) => {
                                use syn::spanned::Spanned;
                                return Err(syn::Error::new(
                                    b.span(),
                                    "Binding on an associated type for trait being implemented is not supported.",
                                )
                                .into());
                            }
                            // TODO: think/check if it should be implemented (I'm not sure)
                            // TODO: try to find an example that should be a valid implementation
                            // in normal rust
                            syn::GenericArgument::Constraint(c) => {
                                use syn::spanned::Spanned;
                                return Err(syn::Error::new(
                                    c.span(),
                                    "Constraints on associated types for trait being implemented is not supported.",
                                )
                                .into());
                            }
                        }
                    }
                }
                // invalid arguments
                syn::PathArguments::Parenthesized(p) => {
                    use syn::spanned::Spanned;
                    return Err(syn::Error::new(
                    p.span(),
                    "arguments for a trait should not be parenthesized. It should be angle-bracketed instead.",
                )
                .into());
                }
            }
            let trait_args_with_state = quote! {
                #(#trait_generics_lifetimes_idents,)*
                #state_ty,
                #(#trait_generics_type_or_const_idents,)*
                #(#trait_generics_const_exprs,)*
            };
            let before_last_segments = trait_path.segments.iter().rev().skip(1).collect::<Vec<_>>();
            let trait_args_with_state_path = quote! {
                #(#before_last_segments::)*#last_segment_ident<#trait_args_with_state>
            };

            let method_generics_lifetimes = self.generics.lifetimes.keys().collect::<Vec<_>>();
            let method_generics_types = self.generics.types.keys().collect::<Vec<_>>();
            let method_generics_consts = self.generics.consts.keys().collect::<Vec<_>>();
            let trait_and_method_arg_idents = quote! {
                #(#trait_generics_lifetimes_idents,)*
                #(#method_generics_lifetimes,)*
                #state_ty
                #(,#method_generics_types)*
                #(,#trait_generics_type_or_const_idents)*
                #(,#trait_generics_const_exprs)*
                #(,#method_generics_consts)*
            };
            let method_arg_idents = quote! {
                // https://github.com/rust-lang/rust/issues/42868
                // TODO: if the function has any late-bound lifetime
                // parameter, then specifying lifetimes is forbidden.
                // if there are only early-bounds and they must be
                // specified, then it could be possible to add an
                // attribute to the impl method item, so that it can
                // indicate wether to exclude (or not-exclude)
                // lifetime params on the method
                //
                // #(#method_generics_lifetimes,)*
                #(#method_generics_types,)*
                #(#method_generics_consts,)*
            };

            let trait_mod = if let Some(trait_mod) = &impl_info.attrs.trait_mod_path {
                trait_mod
            } else {
                use syn::spanned::Spanned;
                return Err(syn::Error::new(
                    impl_info.trait_path.span(),
                    "missing `trait` attribute. TODO: write the trait module path's, then rename the ident itself to lowercase and insert as the last path segment.",
                )
                .into());
            };

            let trait_method_mod = quote!(#trait_mod::#method_mod_name);
            let trait_method_mod_serve_str = format!(
                "{}::{}::Serve",
                quote!(#trait_mod),
                quote!(#method_mod_name)
            );
            let trait_method_mod_request_str = format!(
                "{}::{}::Request",
                quote!(#trait_mod),
                quote!(#method_mod_name)
            );
            let args_link_str = format!("{}::Args", quote!(#trait_method_mod));

            let trait_generic_lifetimes =
                &impl_info.generics.lifetimes.values().collect::<Vec<_>>();
            let trait_generic_types = &impl_info.generics.types.values().collect::<Vec<_>>();
            let trait_generic_consts = &impl_info.generics.consts.values().collect::<Vec<_>>();

            // TODO: test various patterns as arguments
            // eg. (a, b): (bool, u8),
            let args_pats = self
                .inputs
                .args
                .iter()
                .map(|a| a.arg.pat.as_ref())
                .collect::<Vec<_>>();

            let where_clause = {
                let state_ty_as_ident = syn::Ident::new(
                    &quote!(#state_ty).to_string(),
                    proc_macro2::Span::call_site(),
                );
                let impl_generics = impl_info
                    .generics
                    .clone()
                    .replace_from_self_to_ident(&state_ty_as_ident);
                let method_generics = self
                    .generics
                    .clone()
                    .replace_from_self_to_ident(&state_ty_as_ident);
                let impl_lifetime_where_clauses =
                    impl_generics.lifetime_bounds.values().collect::<Vec<_>>();
                let impl_type_where_clauses =
                    impl_generics.type_bounds.values().collect::<Vec<_>>();

                let method_lifetime_where_clauses =
                    method_generics.lifetime_bounds.values().collect::<Vec<_>>();
                let method_type_where_clauses =
                    method_generics.type_bounds.values().collect::<Vec<_>>();

                quote! {
                    where
                        // implicit bound is not required since it was
                        // already implicitly added as a method's bound
                        #(#impl_lifetime_where_clauses,)*
                        #(#method_lifetime_where_clauses,)*
                        #(#method_type_where_clauses,)*
                        #(#impl_type_where_clauses,)*
                }
            };

            let (return_ident, return_type, return_value) = match &self.ret {
                syn::ReturnType::Default => (
                    //
                    quote!(()),
                    quote!(()),
                    quote!(None),
                ),
                syn::ReturnType::Type(_t, ty) => (
                    //
                    quote!(ret),
                    quote!(#ty),
                    quote! {
                        let ret = #trait_method_mod::Return::<
                            #trait_and_method_arg_idents
                        >(ret, Default::default());
                        Some(ret)
                    },
                ),
            };

            let receiver_kind = &self.inputs.receiver_kind;
            let receiver_kind_trait_name = receiver_kind.quote_trait_name();
            let receiver_kind_state = receiver_kind.quote_self_argument();
            let receiver_kind_extern_serve = match receiver_kind {
                inputs::ReceiverKind::RefMut => quote! {
                    fn extern_serve() {
                        use _interface::ServeRefMut;
                        let method_wrapper = |state: &mut Self::State, args: Self::Args| {
                            let #return_ident: #return_type = <Self::State as #trait_path>::#original_method_ident::< //
                                #method_arg_idents
                            > (state, #(args.#args_pats),*);
                            #return_value
                        };
                        Self::serve(method_wrapper);
                    }
                },
                inputs::ReceiverKind::Ref => quote! {
                    fn extern_serve() {
                        use _interface::ServeRef;
                        let method_wrapper = |state: &Self::State, args: Self::Args| {
                            let #return_ident: #return_type = <Self::State as #trait_path>::#original_method_ident::< //
                                #method_arg_idents
                            > (state, #(args.#args_pats),*);
                            #return_value
                        };
                        Self::serve(method_wrapper);
                    }
                },
                inputs::ReceiverKind::Owned => quote! {
                    fn extern_serve() {
                        use _interface::ServeOwned;
                        let method_wrapper = |state: Self::State, args: Self::Args| {
                            let #return_ident: #return_type = <Self::State as #trait_path>::#original_method_ident::< //
                                #method_arg_idents
                            > (state, #(args.#args_pats),*);
                            #return_value
                        };
                        Self::serve(method_wrapper);
                    }
                },
                inputs::ReceiverKind::Stateless => quote! {
                    fn extern_serve() {
                        use _interface::ServeStateless;
                        let method_wrapper = |args: Self::Args| {
                            let #return_ident: #return_type = <Self::State as #trait_path>::#original_method_ident::< //
                                #method_arg_idents
                            > (#(args.#args_pats),*);
                            #return_value
                        };
                        Self::serve(method_wrapper);
                    }
                },
            };

            let interface_serve = if matches!(impl_info.attrs.serve, Some(true) | None) {
                quote! {
                    #[doc = #doc_generated]
                    #[doc = ""]
                    #[doc = " This implementation defines some typing information required by [`interface::Serve`](_interface::Serve)."]
                    #[doc = ""]
                    #(#attr_docs)*
                    impl < //
                        #(#trait_generic_lifetimes,)*
                        #(#method_generics_lifetimes,)*
                        #(#method_generics_types,)*
                        #(#trait_generic_types,)*
                        #(#trait_generic_consts,)*
                        #(#method_generics_consts,)*
                    > _interface::Serve< //
                        _interface::Json,
                        _interface::Json
                    > //
                    for  #trait_method_mod::serve::Serve<#trait_and_method_arg_idents>
                    #where_clause
                    {
                        type State = #state_ty;
                        type Args = #trait_method_mod::Args<#trait_and_method_arg_idents>;
                        type Return = #trait_method_mod::Return< //
                            #trait_and_method_arg_idents
                        >;
                    }

                }
            } else {
                quote!()
            };

            let interface_args_serve = if matches!(impl_info.attrs.serve, Some(true) | None) {
                quote! {
                    #[doc = #doc_generated]
                    #[doc = ""]
                    #[doc = " This implementation prepares the [`Args`]("]
                    #[doc = #args_link_str]
                    #[doc = ") that will be sent into the method."]
                    #[doc = ""]
                    #(#attr_docs)*
                    impl < //
                        #(#trait_generic_lifetimes,)*
                        #(#method_generics_lifetimes,)*
                        #(#method_generics_types,)*
                        #(#trait_generic_types,)*
                        #(#trait_generic_consts,)*
                        #(#method_generics_consts,)*
                    > #receiver_kind_trait_name< //
                        _interface::Json,
                        _interface::Json
                    > //
                    for  #trait_method_mod::serve::Serve<#trait_and_method_arg_idents>
                    #where_clause
                    {
                        type Method = fn(#receiver_kind_state Self::Args) -> Option<Self::Return>;

                        #receiver_kind_extern_serve
                    }
                }
            } else {
                quote!()
            };

            let serve_shortcut_type = if matches!(impl_info.attrs.serve, Some(true) | None) {
                quote! {
                    #[doc = #doc_generated]
                    #[doc = ""]
                    #[doc = " Specializes the `_State` of [`"]
                    #[doc = #trait_method_mod_serve_str]
                    #[doc = "`] as the struct [`"]
                    #[doc = #self_ty_str]
                    #[doc = ".  "]
                    #[doc = ""]
                    #(#attr_docs)*
                    pub type Serve<
                        #(#trait_generic_lifetimes,)*
                        #(#method_generics_lifetimes,)*
                        #(#method_generics_types,)*
                        #(#trait_generic_types,)*
                        #(#trait_generic_consts,)*
                        #(#method_generics_consts,)*
                    > = #trait_method_mod::serve::Serve<#trait_and_method_arg_idents>;
                }
            } else {
                quote!()
            };

            let request_shortcut_type = quote! {
                #[doc = #doc_generated]
                #[doc = ""]
                #[doc = " Specializes the `_State` of [`"]
                #[doc = #trait_method_mod_request_str]
                #[doc = "`] as the struct [`"]
                #[doc = #self_ty_str]
                #[doc = "`].  "]
                #[doc = ""]
                #(#attr_docs)*
                pub type Request<
                    #(#trait_generic_lifetimes,)*
                    #(#method_generics_lifetimes,)*
                    #(#method_generics_types,)*
                    #(#trait_generic_types,)*
                    #(#trait_generic_consts,)*
                    #(#method_generics_consts,)*
                > = #trait_method_mod::request::Request<#trait_and_method_arg_idents>;
            };

            quote! {
                #[doc = #doc_generated]
                #[doc = ""]
                #(#attr_docs)*
                #[allow(non_camel_case_types)]
                pub mod #method_mod_name {
                    use super::*;
                    use #internal_interface as _interface;

                    #interface_serve

                    #interface_args_serve

                    #serve_shortcut_type

                    #request_shortcut_type
                }

            }
        } else {
            quote! {}
        };
        Ok(res)
        // panic!("{}", res.unwrap())
    }
}
