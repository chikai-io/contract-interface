use crate::core_impl::info_extractor::{
    attr_sig_info_called_in::AttrSigInfo, impl_item_method_info_called_in::ImplItemMethodInfo,
    InputStructType, MethodType, SerializerType,
};
use crate::error;
use crate::info_extractor::item_impl_info_called_in::ItemImplInfo;
use quote::quote;
use syn::export::TokenStream2;
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

        let doc_generated = if let Some(trait_path) = &impl_info.trait_path {
            // https://github.com/rust-lang/rust/issues/74563
            //
            // TODO: currently it's not possible to link directly to
            // the implementation's documentation itself, so both
            // the trait and the struct are referred
            format!(
                " Generated code based on the implementation of [`{}::{}()`] for [`{}`].",
                quote! {#trait_path},
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
            let trait_and_methods_arg_idents = quote! {
                #(#trait_generics_lifetimes_idents,)*
                #(#method_generics_lifetimes,)*
                #state_ty
                #(,#method_generics_types)*
                #(,#trait_generics_type_or_const_idents)*
                #(,#trait_generics_const_exprs)*
                #(,#method_generics_consts)*
            };

            let trait_mod = if let Some(trait_mod) = &impl_info.attrs.trait_mod_path {
                trait_mod
            } else {
                use syn::spanned::Spanned;
                return Err(syn::Error::new(
                    impl_info.trait_path.span(),
                    "TODO: write the trait module path's, then rename the ident itself to lowercase and insert as the last path segment.",
                )
                .into());
            };

            let trait_method_mod = quote!(#trait_mod::#method_mod_name);

            let trait_generic_lifetimes = impl_info.generics.lifetimes.values();
            let trait_generic_types = impl_info.generics.types.values();
            let trait_generic_consts = impl_info.generics.consts.values();

            let args_pats = self
                .inputs
                .args
                .iter()
                .map(|a| a.arg.pat.as_ref())
                .collect::<Vec<_>>();

            quote! {
                #[doc = #doc_generated]
                #[doc = ""]
                #(#attr_docs)*
                #[allow(non_camel_case_types)]
                pub mod #method_mod_name {
                    use super::*;
                    use #internal_interface as _interface;

                    #[doc = #doc_generated]
                    #[doc = ""]
                    impl < //
                        #(#trait_generic_lifetimes,)*
                        #(#method_generics_lifetimes,)*
                        #(#method_generics_types,)*
                        #(#trait_generic_types,)*
                        #(#trait_generic_consts,)*
                        #(#method_generics_consts,)*
                    > _interface::CalledIn< //
                        _interface::Json,
                        _interface::Json
                    > //
                    for  #trait_method_mod::CalledIn<#trait_and_methods_arg_idents> {
                        type State = #state_ty;
                        type Args = #trait_method_mod::Args<#trait_and_methods_arg_idents>;
                        type Return = #trait_method_mod::Return<()>;
                        type Method = fn(&mut Self::State, Self::Args) -> Option<Self::Return>;

                        fn exposed_called_in() {
                            let method_wrapper = |state: &mut Self::State, args: Self::Args| {
                                let () = <Self::State as #trait_path>::#original_method_ident(state, #(args.#args_pats),*);
                                None
                            };
                            Self::called_in(method_wrapper);
                        }
                    }
                }

            }
        } else {
            quote! {}
        };
        Ok(res)
        // panic!("{}", res.unwrap())

        /*
        let ImplItemMethodInfo { attr_signature_info, struct_type, .. } = self;
        // Args provided by `env::input()`.
        let has_input_args = attr_signature_info.input_args().next().is_some();

        let panic_hook = quote! {
            near_sdk::env::setup_panic_hook();
        };
        let arg_struct;
        let arg_parsing;
        if has_input_args {
            arg_struct = attr_signature_info.input_struct(InputStructType::Deserialization);
            let decomposition = attr_signature_info.decomposition_pattern();
            let serializer_invocation = match attr_signature_info.input_serializer {
                SerializerType::JSON => quote! {
                    near_sdk::serde_json::from_slice(
                        &near_sdk::env::input().expect("Expected input since method has arguments.")
                    ).expect("Failed to deserialize input from JSON.")
                },
                SerializerType::Borsh => quote! {
                    near_sdk::borsh::BorshDeserialize::try_from_slice(
                        &near_sdk::env::input().expect("Expected input since method has arguments.")
                    ).expect("Failed to deserialize input from Borsh.")
                },
            };
            arg_parsing = quote! {
                let #decomposition : Input = #serializer_invocation ;
            };
        } else {
            arg_struct = TokenStream2::new();
            arg_parsing = TokenStream2::new();
        };

        let callback_deser = attr_signature_info.callback_deserialization();
        let callback_vec_deser = attr_signature_info.callback_vec_deserialization();

        let arg_list = attr_signature_info.arg_list();
        let AttrSigInfo {
            non_bindgen_attrs,
            ident,
            receiver,
            returns,
            result_serializer,
            method_type,
            is_payable,
            is_private,
            ..
        } = attr_signature_info;
        let deposit_check = if *is_payable || matches!(method_type, &MethodType::View) {
            // No check if the method is payable or a view method
            quote! {}
        } else {
            // If method is not payable, do a check to make sure that it doesn't consume deposit
            let error = format!("Method {} doesn't accept deposit", ident.to_string());
            quote! {
                if near_sdk::env::attached_deposit() != 0 {
                    near_sdk::env::panic_str(#error);
                }
            }
        };
        let is_private_check = if *is_private {
            let error = format!("Method {} is private", ident.to_string());
            quote! {
                if near_sdk::env::current_account_id() != near_sdk::env::predecessor_account_id() {
                    near_sdk::env::panic_str(#error);
                }
            }
        } else {
            quote! {}
        };
        let body = if matches!(method_type, &MethodType::Init) {
            quote! {
                if near_sdk::env::state_exists() {
                    near_sdk::env::panic_str("The contract has already been initialized");
                }
                let contract = #struct_type::#ident(#arg_list);
                near_sdk::env::state_write(&contract);
            }
        } else if matches!(method_type, &MethodType::InitIgnoreState) {
            quote! {
                let contract = #struct_type::#ident(#arg_list);
                near_sdk::env::state_write(&contract);
            }
        } else {
            let contract_deser;
            let method_invocation;
            let contract_ser;
            if let Some(receiver) = receiver {
                let mutability = &receiver.mutability;
                contract_deser = quote! {
                    let #mutability contract: #struct_type = near_sdk::env::state_read().unwrap_or_default();
                };
                method_invocation = quote! {
                    contract.#ident(#arg_list)
                };
                if matches!(method_type, &MethodType::Regular) {
                    contract_ser = quote! {
                        near_sdk::env::state_write(&contract);
                    };
                } else {
                    contract_ser = TokenStream2::new();
                }
            } else {
                contract_deser = TokenStream2::new();
                method_invocation = quote! {
                    #struct_type::#ident(#arg_list)
                };
                contract_ser = TokenStream2::new();
            }
            match returns {
                ReturnType::Default => quote! {
                    #contract_deser
                    #method_invocation;
                    #contract_ser
                },
                ReturnType::Type(_, _) => {
                    let value_ser = match result_serializer {
                        SerializerType::JSON => quote! {
                            let result = near_sdk::serde_json::to_vec(&result).expect("Failed to serialize the return value using JSON.");
                        },
                        SerializerType::Borsh => quote! {
                            let result = near_sdk::borsh::BorshSerialize::try_to_vec(&result).expect("Failed to serialize the return value using Borsh.");
                        },
                    };
                    quote! {
                    #contract_deser
                    let result = #method_invocation;
                    #value_ser
                    near_sdk::env::value_return(&result);
                    #contract_ser
                    }
                }
            }
        };
        let non_bindgen_attrs = non_bindgen_attrs.iter().fold(TokenStream2::new(), |acc, value| {
            quote! {
                #acc
                #value
            }
        });
        quote! {
            #non_bindgen_attrs
            #[cfg(target_arch = "wasm32")]
            #[no_mangle]
            pub extern "C" fn #ident() {
                #panic_hook
                #is_private_check
                #deposit_check
                #arg_struct
                #arg_parsing
                #callback_deser
                #callback_vec_deser
                #body
            }
        }
        */
    }

    pub fn marshal_method(&self) -> error::Result<TokenStream2> {
        /*
        let ImplItemMethodInfo {
            attr_signature_info,
            ..
        } = self;
        let has_input_args = attr_signature_info.input_args().next().is_some();

        let pat_type_list = attr_signature_info.pat_type_list();
        let serialize_args = if has_input_args {
            match &attr_signature_info.input_serializer {
                SerializerType::Borsh => {
                    crate::info_extractor::trait_item_method_info_called_in::TraitItemMethodInfo::generate_serialier(
                        attr_signature_info,
                        &attr_signature_info.input_serializer,
                    )
                }
                SerializerType::JSON => json_serialize(attr_signature_info),
            }
        } else {
            quote! {
             let args = vec![];
            }
        };

        let AttrSigInfo {
            non_bindgen_attrs,
            ident,
            // receiver,
            // returns,
            // result_serializer,
            // is_init,
            method_type,
            original_sig,
            ..
        } = attr_signature_info;
        let return_ident = quote! { -> near_sdk::PendingContractTx };
        let params = quote! {
            &self, #pat_type_list
        };
        let ident_str = ident.to_string();
        let is_view = if matches!(method_type, MethodType::View) {
            quote! {true}
        } else {
            quote! {false}
        };

        let non_bindgen_attrs = non_bindgen_attrs
            .iter()
            .fold(TokenStream2::new(), |acc, value| {
                quote! {
                    #acc
                    #value
                }
            });
        let Signature { generics, .. } = original_sig;
        quote! {
            #[cfg(not(target_arch = "wasm32"))]
            #non_bindgen_attrs
            pub fn #ident#generics(#params) #return_ident {
                #serialize_args
                near_sdk::PendingContractTx::new_from_bytes(self.account_id.clone(), #ident_str, args, #is_view)
            }
        }
        */
        Ok(quote! {})
    }
}

fn json_serialize(attr_signature_info: &AttrSigInfo) -> error::Result<TokenStream2> {
    let args: TokenStream2 = attr_signature_info
        .input_args()
        .fold(None, |acc: Option<TokenStream2>, value| {
            let ident = &value.ident;
            let ident_str = ident.to_string();
            Some(match acc {
                None => quote! { #ident_str: #ident },
                Some(a) => quote! { #a, #ident_str: #ident },
            })
        })
        .unwrap();
    Ok(quote! {
      let args = near_sdk::serde_json::json!({#args}).to_string().into_bytes();
    })
}
