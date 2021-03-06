use crate::core_impl::info_extractor::impl_item_method_info::ImplItemMethodInfo;
use crate::error;
use crate::info_extractor::{inputs, item_impl_info::ItemImplInfo};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl ImplItemMethodInfo {
    /// Generate wrapper method for the given method of the contract.
    pub fn method_wrapper(
        &self,
        original_method_ident: &syn::Ident,
        impl_info: &ItemImplInfo,
    ) -> error::Result<TokenStream2> {
        use inputs::ReceiverKind;

        let internal_interface = crate::crate_name("contract-interface")?;
        let near_sdk = crate::crate_name("near-sdk")?;

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
            let mut trait_generics_lifetimes_idents = vec![];
            let mut trait_generics_type_or_const_idents = vec![];
            let mut trait_generics_const_exprs = vec![];
            match &last_segment.arguments {
                // no additional generics
                syn::PathArguments::None => {}
                // some generics. Needs to separate the lifetime from the rest,
                // because the state shall be placed as the first parameter
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

            // let last_segment_ident = &last_segment.ident;
            // let trait_args_with_state = quote! {
            //     #(#trait_generics_lifetimes_idents,)*
            //     #state_ty,
            //     #(#trait_generics_type_or_const_idents,)*
            //     #(#trait_generics_const_exprs,)*
            // };
            // let before_last_segments = trait_path.segments.iter().rev().skip(1).collect::<Vec<_>>();
            // let trait_args_with_state_path = quote! {
            //     #(#before_last_segments::)*#last_segment_ident<#trait_args_with_state>
            // };

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

            let impl_generic_lifetimes = &impl_info.generics.lifetimes.values().collect::<Vec<_>>();
            let impl_generic_types = &impl_info.generics.types.values().collect::<Vec<_>>();
            let impl_generic_consts = &impl_info.generics.consts.values().collect::<Vec<_>>();

            // TODO: test various patterns as arguments
            // eg. (a, b): (bool, u8),
            let args_pats = self
                .inputs
                .args
                .iter()
                .map(|a| {
                    let pat = a.arg.pat.as_ref();
                    let s = quote! {args.#pat};
                    if let Some(fake) = a.fake_arg.as_ref() {
                        let m = &fake.modification;
                        m.modify_pat(s)
                    } else {
                        s
                    }
                })
                .collect::<Vec<_>>();

            let where_clause = {
                let impl_generics = impl_info
                    .generics
                    .clone()
                    .replace_from_self_to_type(state_ty);
                let method_generics = self.generics.clone().replace_from_self_to_type(state_ty);

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

            let recv_kind = &self.inputs.receiver_kind;

            let (return_ident, return_type, return_value);
            match &self.ret {
                syn::ReturnType::Default => {
                    //
                    return_ident = quote!(());
                    return_type = quote!(());
                    return_value = quote!(None);
                }
                syn::ReturnType::Type(_t, ty) => {
                    //
                    return_ident = quote!(ret);
                    return_type = {
                        if matches!(
                            recv_kind,
                            inputs::ReceiverKind::Owned | inputs::ReceiverKind::StatelessInit
                        ) {
                            quote!(#ty::State)
                        } else {
                            quote!(#ty)
                        }
                    };

                    let return_value_tmp = quote! {
                        let ret = #trait_method_mod::Return::<
                            #trait_and_method_arg_idents
                        >
                        {
                            value: ret,
                            _phantom: Default::default()
                        };
                    };
                    return_value =
                        if matches!(recv_kind, ReceiverKind::Owned | ReceiverKind::StatelessInit) {
                            quote! {
                                #return_value_tmp
                                ret.value
                            }
                        } else {
                            quote! {
                                #return_value_tmp
                                Some(ret)
                            }
                        };
                }
            };

            let outer_type_where_clause = match recv_kind {
                ReceiverKind::RefMut => {
                    quote! {
                        where
                            OuterType: _near_sdk::borsh::BorshDeserialize + _near_sdk::borsh::BorshSerialize
                    }
                }
                ReceiverKind::Ref => quote! {
                    where
                        OuterType: _near_sdk::borsh::BorshDeserialize
                },
                ReceiverKind::Owned => quote! {
                    where
                        OuterType: _near_sdk::borsh::BorshDeserialize + _near_sdk::borsh::BorshSerialize
                },
                ReceiverKind::Stateless => quote!(),
                ReceiverKind::StatelessInit => quote! {
                    where
                        OuterType: _near_sdk::borsh::BorshSerialize,
                        Self::State: Into<OuterType>
                },
            };

            let serve_fn = {
                // init_ignore_state is false if it's set to false or if it's missing
                let init_ignore_state = self
                    .attrs
                    .init
                    .as_ref()
                    .and_then(|i| i.ignore_state)
                    .unwrap_or_default();

                let init_check =
                    if matches!(recv_kind, ReceiverKind::StatelessInit) && !init_ignore_state {
                        quote!(Self::panic_on_already_existing_state();)
                    } else {
                        quote!()
                    };

                let payable = match (&self.inputs.receiver_kind, self.attrs.payable) {
                    // methods can are Call (non-View) and forbids payments
                    (
                        ReceiverKind::RefMut | ReceiverKind::Owned | ReceiverKind::StatelessInit,
                        false,
                    ) => quote! {
                        Self::panic_on_deposit();
                    },
                    // methods can are Call (non-View) and allows payments
                    (
                        ReceiverKind::RefMut | ReceiverKind::Owned | ReceiverKind::StatelessInit,
                        true,
                    ) => quote!(),
                    // methods that are View cannot verify if there is
                    // some attached payment (env::attached_deposit),
                    // so there is no need to verify it
                    //
                    // TODO: when setting #[contract(payable)], deny that
                    // attribute itself for view methods
                    (_, _) => quote!(),
                };

                // TODO: decide whether all methods should be private by default,
                // which tends to be safer - currently they are not
                let private = if self.attrs.private {
                    quote!(Self::panic_on_non_private();)
                } else {
                    quote!()
                };

                // TODO: consider different types of args
                // (like the sdk: regular, callback, etc)

                let args_empty = self.inputs.args.is_empty();

                // let args = #args_expr;
                let args_expr = if args_empty {
                    // empty args still requires some blank structure
                    // that holds typing information
                    //
                    // so even if no args are required, that (zero-sized) value is
                    // still passed around (as required by the trait definitions)
                    quote! {
                        Self::Args {
                            _phantom: Default::default(),
                        }
                    }
                } else {
                    quote!(Self::deserialize_args_from_input())
                };

                // note: state initialization based on Defaults are completely disallowed.
                // all states that will be initialized and saved must be explicitly init
                //
                // the only situation where a state can be created from a default impl
                // is on a ref self method, and only if it has a
                // #[contract(allow_temporary_state)], in which case that default state will
                // stil not even be stored.
                let state_load = match recv_kind {
                    ReceiverKind::RefMut => {
                        quote! {
                            let mut contract: OuterType = Self::state_read_or_panic::<OuterType>();
                            let state: &mut Self::State = access(&mut contract);
                        }
                    }
                    ReceiverKind::Ref => {
                        if self.attrs.allow_temporary_state {
                            quote! {
                                let mut contract: OuterType = Self::state_read_or_default::<OuterType>();
                                let mut state: &Self::State = access(&contract);
                            }
                        } else {
                            quote! {
                                let mut contract: OuterType = Self::state_read_or_panic::<OuterType>();
                                let mut state: &Self::State = access(&contract);
                            }
                        }
                    }
                    ReceiverKind::Owned => {
                        quote! {
                            let mut contract: OuterType = Self::state_read_or_panic();
                            // TODO:
                            // hope that the compiler will optmize the clone away
                            let mut state: Self::State = access(&mut contract).clone();
                        }
                    }
                    ReceiverKind::Stateless => quote! {
                        let _contract = ();
                        let _state = ();
                    },

                    // just declare the state and set it's type,
                    // it will be initialized from the method's result later on
                    ReceiverKind::StatelessInit => quote!(
                        let contract: OuterType;
                        let state: Self::State;
                    ),
                };

                // let result = method(#method_params);
                let method_params = match recv_kind {
                    ReceiverKind::Stateless => quote!(args),
                    ReceiverKind::StatelessInit => quote!(args),
                    ReceiverKind::RefMut => quote!(state, args),
                    ReceiverKind::Ref => quote!(state, args),
                    ReceiverKind::Owned => quote!(state, args),
                };

                let result_serialize =
                    if matches!(recv_kind, ReceiverKind::Owned | ReceiverKind::StatelessInit) {
                        // init and owned are required to return exactly a State
                        // so they can never have other output values
                        quote!()
                    } else {
                        // other kind of methods can have output normally
                        quote!(Self::may_serialize_return_as_output(result);)
                    };

                let state_write = match recv_kind {
                    // ref mut self always (over)writes state
                    ReceiverKind::RefMut => {
                        quote! {
                            Self::state_write::<OuterType>(&contract);
                        }
                    }

                    // ref self never (over)writes state
                    ReceiverKind::Ref => quote!(),

                    // owned always overwrites state, but will give compile-error
                    // if returned value is not a state
                    ReceiverKind::Owned => quote! {
                        *access(&mut contract) = result;
                        Self::state_write::<OuterType>(&contract);
                    },

                    // stateless methods never (over)writes state
                    ReceiverKind::Stateless => quote!(),

                    // init always overwrites state, but will give compile-error
                    // if returned value is not a state
                    ReceiverKind::StatelessInit => quote! {
                        contract = result.into();
                        Self::state_write::<OuterType>(&contract);
                    },
                };

                let fn_args = match recv_kind {
                    ReceiverKind::RefMut => {
                        quote! {
                            access: fn(&mut OuterType) -> &mut Self::State,
                            method: Self::Method
                        }
                    }
                    ReceiverKind::Ref => quote! {
                        access: fn(&OuterType) -> &Self::State,
                        method: Self::Method
                    },
                    ReceiverKind::Owned => quote! {
                        access: fn(&mut OuterType) -> &mut Self::State,
                        method: Self::Method
                    },
                    ReceiverKind::Stateless => quote! {
                        method: Self::Method
                    },
                    ReceiverKind::StatelessInit => quote! {
                        method: Self::Method
                    },
                };

                let serve_generics = match recv_kind {
                    ReceiverKind::RefMut => quote!(OuterType),
                    ReceiverKind::Ref => quote!(OuterType),
                    ReceiverKind::Owned => quote!(OuterType),
                    ReceiverKind::Stateless => quote!(),
                    ReceiverKind::StatelessInit => quote!(OuterType),
                };

                quote! {
                    fn serve<#serve_generics>(
                        #fn_args
                    )
                    #outer_type_where_clause
                    {
                        use _interface::Serve as _;
                        Self::setup_panic_hook();
                        #init_check
                        #payable
                        #private
                        let args = #args_expr;
                        #state_load
                        let result = method(#method_params);
                        #result_serialize
                        #state_write
                    }
                }
            };

            let receiver_kind = &self.inputs.receiver_kind;
            let receiver_kind_trait_name = receiver_kind.quote_trait_name();
            let receiver_kind_state = receiver_kind.quote_self_argument();
            let receiver_kind_extern_serve = match receiver_kind {
                inputs::ReceiverKind::RefMut => quote! {
                    fn extern_serve<OuterType>(
                        access: fn(&mut OuterType) -> &mut Self::State
                    )
                    #outer_type_where_clause
                    {
                        use _interface::ServeRefMut;
                        let method_wrapper = |state: &mut Self::State, mut args: Self::Args| {
                            let #return_ident: #return_type = <Self::State as #trait_path>::#original_method_ident::< //
                                #method_arg_idents
                            > (state, #(#args_pats),*);
                            #return_value
                        };
                        Self::serve::<OuterType>(access, method_wrapper);
                    }
                },
                inputs::ReceiverKind::Ref => quote! {
                    fn extern_serve<OuterType>(
                        access: fn(&OuterType) -> &Self::State
                    )
                    #outer_type_where_clause
                    {
                        use _interface::ServeRef;
                        let method_wrapper = |state: &Self::State, mut args: Self::Args| {
                            let #return_ident: #return_type = <Self::State as #trait_path>::#original_method_ident::< //
                                #method_arg_idents
                            > (state, #(#args_pats),*);
                            #return_value
                        };
                        Self::serve::<OuterType>(access, method_wrapper);
                    }
                },
                inputs::ReceiverKind::Owned => quote! {
                    fn extern_serve<OuterType>(
                        access: fn(&mut OuterType) -> &mut Self::State
                    )
                    #outer_type_where_clause
                    {
                        use _interface::ServeOwned;
                        let method_wrapper = |state: Self::State, mut args: Self::Args| {
                            let #return_ident: #return_type = <Self::State as #trait_path>::#original_method_ident::< //
                                #method_arg_idents
                            > (state, #(#args_pats),*);
                            #return_value
                        };
                        Self::serve::<OuterType>(access, method_wrapper);
                    }
                },
                inputs::ReceiverKind::Stateless => quote! {
                    fn extern_serve()
                    {
                        use _interface::ServeStateless;
                        let method_wrapper = |mut args: Self::Args| {
                            let #return_ident: #return_type = <Self::State as #trait_path>::#original_method_ident::< //
                                #method_arg_idents
                            > (#(#args_pats),*);
                            #return_value
                        };
                        Self::serve(method_wrapper);
                    }
                },
                inputs::ReceiverKind::StatelessInit => quote! {
                    fn extern_serve<OuterType>()
                    #outer_type_where_clause
                    {
                        use _interface::ServeStatelessInit;
                        let method_wrapper = |mut args: Self::Args| {
                            let #return_ident: #return_type = <Self::State as #trait_path>::#original_method_ident::< //
                                #method_arg_idents
                            > (#(#args_pats),*);
                            #return_value
                        };
                        Self::serve::<OuterType>(method_wrapper);
                    }
                },
            };

            let interface_serve = if impl_info.attrs.serve {
                let recv_kind = &self.inputs.receiver_kind;
                let result_serializer = if matches!(
                    recv_kind,
                    inputs::ReceiverKind::Owned | inputs::ReceiverKind::StatelessInit
                ) {
                    quote! {
                        _interface::Borsh,
                    }
                } else {
                    quote! {
                        _interface::Json,
                    }
                };
                quote! {
                    #[doc = #doc_generated]
                    #[doc = ""]
                    #[doc = " This implementation defines some typing information required by [`interface::Serve`](_interface::Serve)."]
                    #[doc = ""]
                    #(#attr_docs)*
                    impl < //
                        #(#impl_generic_lifetimes,)*
                        #(#method_generics_lifetimes,)*
                        #(#method_generics_types,)*
                        #(#impl_generic_types,)*
                        #(#impl_generic_consts,)*
                        #(#method_generics_consts,)*
                    > _interface::Serve< //
                        _interface::Json,
                        #result_serializer
                        _Diverger
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

            let interface_args_serve = if impl_info.attrs.serve {
                let recv_kind = &self.inputs.receiver_kind;
                let trait_generic_args = {
                    if matches!(
                        recv_kind,
                        inputs::ReceiverKind::Owned | inputs::ReceiverKind::StatelessInit
                    ) {
                        quote! {
                                _interface::Json,
                                _Diverger
                        }
                    } else {
                        quote! {
                                _interface::Json,
                                _interface::Json,
                                _Diverger
                        }
                    }
                };
                let method_type_return = {
                    if matches!(
                        recv_kind,
                        inputs::ReceiverKind::Owned | inputs::ReceiverKind::StatelessInit
                    ) {
                        quote!(Self::State)
                    } else {
                        quote!(Option<Self::Return>)
                    }
                };
                quote! {
                    #[doc = #doc_generated]
                    #[doc = ""]
                    #[doc = " This implementation prepares the [`Args`]("]
                    #[doc = #args_link_str]
                    #[doc = ") that will be sent into the method."]
                    #[doc = ""]
                    #(#attr_docs)*
                    impl < //
                        #(#impl_generic_lifetimes,)*
                        #(#method_generics_lifetimes,)*
                        #(#method_generics_types,)*
                        #(#impl_generic_types,)*
                        #(#impl_generic_consts,)*
                        #(#method_generics_consts,)*
                    > #receiver_kind_trait_name< //
                        #trait_generic_args
                    > //
                    for  #trait_method_mod::serve::Serve<#trait_and_method_arg_idents>
                    #where_clause
                    {
                        type Method = fn(#receiver_kind_state Self::Args) -> #method_type_return;

                        #receiver_kind_extern_serve

                        #serve_fn
                    }
                }
            } else {
                quote!()
            };

            let serve_shortcut_type = if impl_info.attrs.serve {
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
                        #(#impl_generic_lifetimes,)*
                        #(#method_generics_lifetimes,)*
                        #(#method_generics_types,)*
                        #(#impl_generic_types,)*
                        #(#impl_generic_consts,)*
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
                    #(#impl_generic_lifetimes,)*
                    #(#method_generics_lifetimes,)*
                    #(#method_generics_types,)*
                    #(#impl_generic_types,)*
                    #(#impl_generic_consts,)*
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
                    use #near_sdk as _near_sdk;

                    #interface_serve

                    #interface_args_serve

                    #serve_shortcut_type

                    #request_shortcut_type

                    #[doc = " Allows specialization of a third-party trait for arbitrary types."]
                    pub struct _Diverger;
                }

            }
        } else {
            quote! {}
        };
        Ok(res)
        // panic!("{}", res.unwrap())
    }
}
